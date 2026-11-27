//! Registro de los iconos visibles en la bandeja.
//!
//! La identidad es el GUID si la app lo dio; si no, su ventana y número.
//! Solo cuenta lo visible: un icono oculto o sin cambios no mueve la
//! revisión, aunque la app reenvíe su aviso sin parar.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::path::Path;
use std::sync::Arc;

use dokka_model::{TrayItem, TrayItemId};
use dokka_scene::Bitmap;
use dokka_win_apps::NameCache;

use crate::notify::{Action, Parsed};
use crate::owner;

/// Quién es cada icono para Dokka.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum TrayKey {
    Guid([u8; 16]),
    Window { hwnd: u32, id: u32 },
}

/// Qué cambia de la imagen con un aviso.
#[derive(Debug)]
pub(crate) enum IconUpdate {
    /// El aviso no habla del icono: se conserva el que había.
    Keep,
    /// Icono nuevo, o `None` si la app lo quitó.
    Set(Option<Arc<Bitmap>>),
}

/// Datos mínimos para reenviar un clic a la app dueña.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ClickTarget {
    pub(crate) hwnd_raw: u32,
    pub(crate) uid: u32,
    pub(crate) callback: u32,
    pub(crate) version: u32,
}

#[derive(Debug)]
struct TrayRecord {
    id: TrayItemId,
    hwnd_raw: u32,
    uid: u32,
    callback: u32,
    version: u32,
    title: String,
    hidden: bool,
    explorer: bool,
    image: Option<Arc<Bitmap>>,
}

/// Estado compartido entre el hilo de la ventana y `TrayHost`.
/// Solo memoria propia y nombres ya leídos: con el candado tomado no se
/// toca ninguna ventana; la ruta del dueño se resuelve en la entrada.
#[derive(Debug)]
pub(crate) struct Shared {
    next_id: u64,
    revision: u64,
    order: Vec<TrayKey>,
    records: HashMap<TrayKey, TrayRecord>,
    names: NameCache,
}

impl Shared {
    pub(crate) fn new() -> Self {
        Self {
            next_id: 1,
            revision: 0,
            order: Vec::new(),
            records: HashMap::new(),
            names: NameCache::default(),
        }
    }

    pub(crate) fn revision(&self) -> u64 {
        self.revision
    }

    /// `true` si ese icono ya está registrado.
    pub(crate) fn knows(&self, parsed: &Parsed) -> bool {
        self.records.contains_key(&TrayKey::from_parsed(parsed))
    }

    /// Aplica un aviso ya leído; `Focus` no cambia nada visible.
    /// `owner_exe` solo llega al aparecer: dice si es del Explorador y da
    /// nombre al icono que llega sin texto.
    pub(crate) fn apply(&mut self, parsed: &Parsed, icon: IconUpdate, owner_exe: Option<&Path>) {
        match parsed.action {
            Action::Add | Action::Modify => self.upsert(parsed, icon, owner_exe),
            Action::Delete => self.remove(&TrayKey::from_parsed(parsed)),
            Action::Version => self.set_version(parsed),
            Action::Focus => {}
        }
    }

    /// Iconos visibles en el orden en que aparecieron, sin los del
    /// Explorador: la lista es de apps en segundo plano.
    pub(crate) fn items(&self) -> Vec<TrayItem> {
        let mut out = Vec::new();
        for key in &self.order {
            if let Some(record) = self.records.get(key)
                && !record.hidden
                && !record.explorer
            {
                out.push(TrayItem {
                    id: record.id,
                    title: record.title.clone(),
                });
            }
        }
        out
    }

    pub(crate) fn image_for(&self, id: TrayItemId) -> Option<Arc<Bitmap>> {
        self.records
            .values()
            .find(|record| record.id == id && !record.hidden)
            .and_then(|record| record.image.clone())
    }

    pub(crate) fn click_for(&self, id: TrayItemId) -> Option<ClickTarget> {
        self.records.values().find_map(|record| {
            if record.id == id && !record.hidden && record.callback != 0 {
                Some(ClickTarget {
                    hwnd_raw: record.hwnd_raw,
                    uid: record.uid,
                    callback: record.callback,
                    version: record.version,
                })
            } else {
                None
            }
        })
    }

    /// Ventanas a comprobar con `IsWindow` fuera del candado.
    pub(crate) fn snapshot_hwnds(&self) -> Vec<(TrayKey, u32)> {
        self.records
            .iter()
            .map(|(key, record)| (*key, record.hwnd_raw))
            .collect()
    }

    /// Quita los muertos; sube la revisión si alguno se veía.
    pub(crate) fn remove_dead(&mut self, dead: &[TrayKey]) {
        if dead.is_empty() {
            return;
        }
        for key in dead {
            if let Some(record) = self.records.remove(key)
                && !record.hidden
                && !record.explorer
            {
                self.bump();
            }
        }
        self.order.retain(|key| !dead.contains(key));
    }

    fn upsert(&mut self, parsed: &Parsed, icon: IconUpdate, owner_exe: Option<&Path>) {
        let key = TrayKey::from_parsed(parsed);
        let fresh = !self.records.contains_key(&key);
        let tip_empty = parsed.tip.as_ref().is_none_or(|tip| tip.trim().is_empty());
        // Solo al aparecer y sin texto: el nombre se lee una vez, no en
        // cada `MODIFY`.
        let fallback = if fresh && tip_empty {
            owner_exe.and_then(|executable| owner::app_title(executable, &mut self.names))
        } else {
            None
        };
        let id = self.next_id;
        let record = match self.records.entry(key) {
            Entry::Occupied(slot) => slot.into_mut(),
            Entry::Vacant(slot) => {
                self.next_id = self.next_id.wrapping_add(1);
                slot.insert(TrayRecord {
                    id: TrayItemId(id),
                    hwnd_raw: parsed.hwnd_raw,
                    uid: parsed.uid,
                    callback: 0,
                    version: 0,
                    title: String::new(),
                    hidden: false,
                    explorer: false,
                    image: None,
                })
            }
        };
        let was_visible = !fresh && !record.hidden && !record.explorer;
        let old_title = record.title.clone();
        record.hwnd_raw = parsed.hwnd_raw;
        record.uid = parsed.uid;
        if let Some(callback) = parsed.callback {
            record.callback = callback;
        }
        if fresh {
            record.explorer = owner_exe.is_some_and(owner::is_explorer);
        }
        if let Some(tip) = &parsed.tip
            && !tip.trim().is_empty()
        {
            record.title = tip.clone();
        }
        if let Some(name) = fallback
            && record.title.is_empty()
        {
            record.title = name;
        }
        if let Some(hidden) = parsed.hidden {
            record.hidden = hidden;
        }
        let mut changed = fresh || record.title != old_title;
        if let IconUpdate::Set(image) = icon {
            let same = match (&record.image, &image) {
                (None, None) => true,
                (Some(old), Some(new)) => old.as_ref() == new.as_ref(),
                _ => false,
            };
            if !same {
                record.image = image;
                changed = true;
            }
        }
        if fresh {
            self.order.push(key);
        }
        let visible = !record.hidden && !record.explorer;
        if was_visible != visible || (visible && changed) {
            self.bump();
        }
    }

    fn remove(&mut self, key: &TrayKey) {
        if let Some(record) = self.records.remove(key) {
            self.order.retain(|known| known != key);
            if !record.hidden && !record.explorer {
                self.bump();
            }
        }
    }

    fn set_version(&mut self, parsed: &Parsed) {
        let key = TrayKey::from_parsed(parsed);
        if let Some(record) = self.records.get_mut(&key)
            && let Some(version) = parsed.version
        {
            record.version = version;
        }
    }

    fn bump(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }
}

impl TrayKey {
    fn from_parsed(parsed: &Parsed) -> Self {
        if let Some(guid) = parsed.guid {
            Self::Guid(guid)
        } else {
            Self::Window {
                hwnd: parsed.hwnd_raw,
                id: parsed.uid,
            }
        }
    }
}
