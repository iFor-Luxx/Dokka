use dokka_model::AppInfo;

/// Minúsculas sin tildes para comparar nombres.
pub(crate) fn normalized(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|letter| match letter {
            'á' | 'à' | 'ä' | 'â' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ñ' => 'n',
            'ç' => 'c',
            other => other,
        })
        .collect()
}

/// Apps cuyo nombre contiene lo buscado, sin distinguir mayúsculas ni tildes.
/// Con la búsqueda vacía salen todas, en el orden que ya traían.
pub fn filter_apps(apps: &[AppInfo], query: &str) -> Vec<AppInfo> {
    let wanted = normalized(query);
    if wanted.is_empty() {
        return apps.to_vec();
    }
    apps.iter()
        .filter(|app| normalized(&app.name).contains(&wanted))
        .cloned()
        .collect()
}
