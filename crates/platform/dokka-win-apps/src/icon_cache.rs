use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dokka_scene::Bitmap;
use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};

use crate::load_icon::load_icon;

/// Iconos ya consultados, para pedir cada uno a Windows una sola vez.
#[derive(Debug)]
pub struct IconCache {
    icons: HashMap<PathBuf, Option<Arc<Bitmap>>>,
}

impl IconCache {
    /// Debe usarse siempre desde el hilo que lo crea.
    pub fn new() -> Self {
        // SAFETY: inicializa COM en este hilo para toda la vida del proceso.
        // Si ya estaba inicializado, la llamada no tiene efecto.
        let _ = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };
        Self {
            icons: HashMap::new(),
        }
    }

    pub fn icon_for(&mut self, executable: &Path) -> Option<Arc<Bitmap>> {
        self.icons
            .entry(executable.to_path_buf())
            .or_insert_with(|| load_icon(executable).ok().map(Arc::new))
            .clone()
    }
}

impl Default for IconCache {
    fn default() -> Self {
        Self::new()
    }
}
