use std::path::Path;

use dokka_model::AppId;

/// Ruta del ejecutable destino cuando se conoce, para no duplicar fijadas.
pub(crate) fn catalog_id(target: Option<&Path>, fallback: &Path) -> AppId {
    AppId::from_executable(target.unwrap_or(fallback))
}

/// Nombre visible: tallo del archivo sin extensión.
pub(crate) fn entry_name(path: &Path) -> Option<String> {
    let name = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())?;
    let name = name.trim();
    (!name.is_empty()).then(|| name.to_owned())
}
