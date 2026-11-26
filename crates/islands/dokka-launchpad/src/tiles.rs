//! Baldosas uniformes del Launchpad, calculadas una vez por apertura.
//!
//! El shell pide las imágenes en cada redibujado; aquí se guarda la baldosa
//! ya tratada por app y se reutiliza mientras siga el mismo `Bitmap`.

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
    pub(crate) fn clear(&mut self) {
        self.entries.clear();
    }

    /// Baldosas para las apps visibles, reutilizando las ya calculadas.
    pub(crate) fn tiled(
        &mut self,
        apps: &[AppInfo],
        images: &HashMap<AppId, Arc<Bitmap>>,
        theme: &Theme,
    ) -> HashMap<AppId, Arc<Bitmap>> {
        let wanted: HashSet<&str> = apps.iter().map(|app| app.id.as_str()).collect();
        self.entries.retain(|id, _| wanted.contains(id.as_str()));
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
