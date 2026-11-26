use std::path::{Path, PathBuf};

/// Carpeta de Windows, para excluir herramientas internas.
fn windows_folder() -> PathBuf {
    let folder = std::env::var_os("WINDIR").unwrap_or_else(|| "C:\\Windows".into());
    PathBuf::from(folder)
}

/// `true` si la ruta está dentro de `%WINDIR%`.
pub(crate) fn is_within_windows(path: &Path) -> bool {
    let folder = windows_folder().to_string_lossy().to_lowercase();
    let text = path.to_string_lossy().to_lowercase();
    text.starts_with(folder.as_str())
}

/// `true` si es el explorador.
pub(crate) fn is_explorer(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().to_lowercase() == "explorer.exe")
        .unwrap_or(false)
}

/// `true` si parece un desinstalador por su nombre.
pub(crate) fn is_uninstaller(path: &Path) -> bool {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    stem.contains("uninstall") || stem.starts_with("unins")
}

/// `true` si el destino vale para el Launchpad: ejecutable fuera de Windows.
pub(crate) fn is_program_target(path: &Path) -> bool {
    let is_exe = path
        .extension()
        .map(|ext| ext.to_string_lossy().to_lowercase() == "exe")
        .unwrap_or(false);
    is_exe && !is_within_windows(path) && !is_explorer(path) && !is_uninstaller(path)
}
