//! Cuántas filas variables trae hoy General.
//!
//! Dos apartados crecen con lo leído: "Idioma y región" con los idiomas de
//! Windows y "Almacenamiento" con los otros discos. Juntarlos en un tipo
//! pequeño evita pasar de 7 argumentos en el golpe y el arrastre.

/// Idiomas y otros discos que hay hoy en General.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ListCounts {
    /// Idiomas preferidos leídos de Windows.
    pub languages: usize,
    /// Otros discos fijos además del disco del sistema.
    pub disks: usize,
}

impl ListCounts {
    /// Crea el par de cuentas de la lista.
    pub fn new(languages: usize, disks: usize) -> Self {
        Self { languages, disks }
    }
}
