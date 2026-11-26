//! Baldosas uniformes del Launchpad, calculadas una vez por app.
//!
//! El shell pide las imágenes en cada redibujado; aquí se guarda la baldosa
//! ya tratada por app y se reutiliza mientras siga el mismo `Bitmap`.
//! La caché sobrevive entre aperturas y solo se podan las apps que ya no están.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use dokka_icons::app_tile;
use dokka_model::{AppId, AppInfo};
use dokka_scene::Bitmap;
use dokka_theme::Theme;

/// Baldosa ya tratada por app, junto a su imagen original.
#[derive(Debug, Default)]
pub(crate) struct TileCache {
    entries: HashMap<AppId, (Arc<Bitmap>, Arc<Bitmap>)>,
}

impl TileCache {
    /// Descarta las apps que ya no están, conservando el resto entre aperturas.
    pub(crate) fn prune(&mut self, apps: &[AppInfo]) {
        let wanted: HashSet<&str> = apps.iter().map(|app| app.id.as_str()).collect();
        self.entries.retain(|id, _| wanted.contains(id.as_str()));
    }

    /// Baldosas para las apps pedidas, reutilizando las ya calculadas.
    /// No poda: se poda una vez al abrir; por fotograma solo se reutiliza.
    pub(crate) fn tiled(
        &mut self,
        apps: &[AppInfo],
        images: &HashMap<AppId, Arc<Bitmap>>,
        theme: &Theme,
    ) -> HashMap<AppId, Arc<Bitmap>> {
        let mut out = HashMap::with_capacity(apps.len());
        for app in apps {
            let Some(source) = images.get(&app.id) else {
                continue;
            };
            if let Some((known, tiled)) = self.entries.get(&app.id)
                && Arc::ptr_eq(known, source)
            {
                out.insert(app.id.clone(), Arc::clone(tiled));
                continue;
            }
            let tiled = app_tile(source, theme).map_or_else(|| Arc::clone(source), Arc::new);
            self.entries
                .insert(app.id.clone(), (Arc::clone(source), Arc::clone(&tiled)));
            out.insert(app.id.clone(), tiled);
        }
        out
    }
}
