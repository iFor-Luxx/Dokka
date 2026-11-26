use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use dokka_contracts::{AppCatalog, ContractError};
use dokka_model::AppInfo;

use crate::desktop_entry::{catalog_id, entry_name};
use crate::desktop_paths::desktop_folders;
use crate::filter_target::{is_program_target, is_uninstaller, is_within_windows};
use crate::read_url::game_protocol;
use crate::resolve_lnk::resolve_lnk;

/// Huella barata del Escritorio: por cada entrada, nombre, fecha y tamaño.
/// Leer el directorio y sus fechas cuesta poco; resolver cada `.lnk` por COM no.
type DesktopFingerprint = Vec<(PathBuf, Option<SystemTime>, u64)>;

#[derive(Debug, Default)]
struct CachedDesktop {
    fingerprint: DesktopFingerprint,
    apps: Vec<AppInfo>,
}

/// Apps del Escritorio para el Launchpad: programas y juegos, sin repetidas.
/// Guarda el último resultado y lo reutiliza mientras el Escritorio no cambie,
/// sin hilos nuevos y sin cambiar el contrato.
#[derive(Debug, Default)]
pub struct DesktopCatalog {
    cache: Mutex<Option<CachedDesktop>>,
}

impl DesktopCatalog {
    pub fn new() -> Self {
        Self::default()
    }

    fn collect_fresh() -> Vec<AppInfo> {
        let mut seen = HashSet::new();
        let mut apps = Vec::new();
        for folder in desktop_folders() {
            let entries = std::fs::read_dir(&folder).into_iter().flatten().flatten();
            for entry in entries {
                if let Some(app) = desktop_app(entry.path().as_path())
                    && seen.insert(app.id.as_str().to_owned())
                {
                    apps.push(app);
                }
            }
        }
        apps
    }
}

/// Huella actual del Escritorio; `None` si no se puede leer con fiabilidad.
fn desktop_fingerprint() -> Option<DesktopFingerprint> {
    let mut fingerprint = Vec::new();
    for folder in desktop_folders() {
        let entries = std::fs::read_dir(&folder).ok()?;
        for entry in entries.flatten() {
            let path = entry.path();
            let metadata = entry.metadata().ok()?;
            if metadata.is_dir() {
                continue;
            }
            fingerprint.push((path, metadata.modified().ok(), metadata.len()));
        }
    }
    fingerprint.sort_by(|left, right| left.0.cmp(&right.0));
    Some(fingerprint)
}

impl AppCatalog for DesktopCatalog {
    fn installed_apps(&self) -> Result<Vec<AppInfo>, ContractError> {
        if let Some(fingerprint) = desktop_fingerprint()
            && let Ok(cached) = self.cache.lock()
            && let Some(known) = cached.as_ref()
            && known.fingerprint == fingerprint
        {
            return Ok(known.apps.clone());
        }
        let apps = Self::collect_fresh();
        if apps.is_empty() && desktop_folders().is_empty() {
            return Err(ContractError::Failed(
                "sin carpetas de escritorio".to_owned(),
            ));
        }
        if let Some(fingerprint) = desktop_fingerprint()
            && let Ok(mut cached) = self.cache.lock()
        {
            *cached = Some(CachedDesktop {
                fingerprint,
                apps: apps.clone(),
            });
        }
        Ok(apps)
    }
}

/// Una entrada del Escritorio convertida en app, si vale para el Launchpad.
fn desktop_app(path: &Path) -> Option<AppInfo> {
    if path.is_dir() {
        return None;
    }
    let extension = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())?;
    match extension.as_str() {
        "lnk" => lnk_app(path),
        "url" => url_app(path),
        "exe" => exe_app(path),
        _ => None,
    }
}

/// Acceso directo: se filtra por el destino pero se abre por el `.lnk`.
fn lnk_app(link: &Path) -> Option<AppInfo> {
    let name = entry_name(link)?;
    let target = resolve_lnk(link)?;
    if !is_program_target(&target) {
        return None;
    }
    Some(AppInfo {
        id: catalog_id(Some(&target), link),
        name,
        executable: link.to_path_buf(),
    })
}

/// Enlace web solo para juegos con protocolo propio.
fn url_app(link: &Path) -> Option<AppInfo> {
    game_protocol(link)?;
    let name = entry_name(link)?;
    Some(AppInfo {
        id: catalog_id(None, link),
        name,
        executable: link.to_path_buf(),
    })
}

/// Ejecutable suelto en el Escritorio.
fn exe_app(path: &Path) -> Option<AppInfo> {
    if is_within_windows(path) || is_uninstaller(path) {
        return None;
    }
    let name = entry_name(path)?;
    Some(AppInfo {
        id: catalog_id(Some(path), path),
        name,
        executable: path.to_path_buf(),
    })
}
