use super::sections::SECTION_NAMES;

/// Minúsculas sin tildes para comparar nombres, como hace el Launchpad: las
/// islas no se importan entre sí, así que se repite aquí su normalización.
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

/// Índices de las secciones cuyo nombre contiene lo buscado, en su orden.
/// Con la búsqueda vacía salen todas.
pub(crate) fn filter_sections(query: &str) -> Vec<usize> {
    let wanted = normalized(query);
    if wanted.is_empty() {
        return (0..SECTION_NAMES.len()).collect();
    }
    SECTION_NAMES
        .iter()
        .enumerate()
        .filter(|(_, name)| normalized(name).contains(&wanted))
        .map(|(index, _)| index)
        .collect()
}
