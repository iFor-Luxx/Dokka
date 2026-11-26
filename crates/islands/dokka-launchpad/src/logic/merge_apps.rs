use std::collections::HashMap;

use dokka_model::{AppId, AppInfo};

/// Une catálogo y fijadas: sin explorador, sin repetidas, ordenadas por nombre.
/// Ante un duplicado se queda la fijada, que eligió la persona.
pub fn merge_apps(catalog: &[AppInfo], pinned: &[AppInfo], explorer: &AppId) -> Vec<AppInfo> {
    let mut kept: HashMap<&str, &AppInfo> = HashMap::new();
    for app in pinned.iter().chain(catalog.iter()) {
        if &app.id == explorer {
            continue;
        }
        kept.entry(app.id.as_str()).or_insert(app);
    }
    let mut apps: Vec<AppInfo> = kept.into_values().cloned().collect();
    apps.sort_by_cached_key(|app| app.name.to_lowercase());
    apps
}
