//! Filtro puro de nombres con tildes y mayúsculas ignoradas.
//!
//! Sin estado: dada la consulta y los nombres, devuelve en orden los índices
//! que contienen la consulta normalizada; primero los que empiezan igual y
//! luego por orden alfabético. Limitado a siete, como el panel.

use dokka_theme::search as measures;

/// Índices de `names` que contienen a `query`, ya ordenados y recortados a
/// lo que cabe en el panel. Con la consulta vacía devuelve los primeros.
pub fn filter_search_order(names: &[String], query: &str) -> Vec<usize> {
    let wanted = normalized(query.trim());
    if wanted.is_empty() {
        return (0..names.len().min(measures::MAX_RESULTS)).collect();
    }
    let mut kept: Vec<(usize, String)> = names
        .iter()
        .enumerate()
        .map(|(index, name)| (index, normalized(name)))
        .filter(|(_, plain)| plain.contains(&wanted))
        .collect();
    kept.sort_by(|left, right| {
        let left_starts = left.1.starts_with(&wanted);
        let right_starts = right.1.starts_with(&wanted);
        right_starts
            .cmp(&left_starts)
            .then_with(|| left.1.cmp(&right.1))
    });
    kept.into_iter()
        .map(|(index, _)| index)
        .take(measures::MAX_RESULTS)
        .collect()
}

/// Minúsculas sin tildes para comparar sin importar cómo se escribió.
fn normalized(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|letter| match letter {
            'á' | 'à' | 'ä' | 'â' | 'ã' | 'å' => 'a',
            'é' | 'è' | 'ë' | 'ê' => 'e',
            'í' | 'ì' | 'ï' | 'î' => 'i',
            'ó' | 'ò' | 'ö' | 'ô' | 'õ' => 'o',
            'ú' | 'ù' | 'ü' | 'û' => 'u',
            'ý' | 'ÿ' => 'y',
            'ç' => 'c',
            'ñ' => 'n',
            rest => rest,
        })
        .collect()
}
