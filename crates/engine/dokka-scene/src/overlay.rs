use crate::{Divider, Dot, Icon, Panel, Pointer, Text, Thumbnail};

/// Botones que deben verse por encima de una miniatura.
#[derive(Debug, Clone, PartialEq)]
pub struct Controls {
    pub panel: Panel,
    pub icons: Vec<Icon>,
}

/// Panel flotante sobre la escena principal.
/// Orden de pintado: panel, píldora, punta, resaltado, líneas, puntos, textos, iconos.
#[derive(Debug, Clone, PartialEq)]
pub struct Overlay {
    pub panel: Panel,
    /// Fondo de la píldora segmentada de navegación, sobre el panel.
    /// Su hover va en `highlight`.
    pub pill: Option<Panel>,
    pub highlight: Option<Panel>,
    pub dividers: Vec<Divider>,
    /// Se dibujan debajo de los textos.
    pub dots: Vec<Dot>,
    pub pointer: Option<Pointer>,
    pub texts: Vec<Text>,
    /// Iconos sobre el resaltado, junto a los textos (el chevrón del menú).
    pub icons: Vec<Icon>,
    pub thumbnails: Vec<Thumbnail>,
    /// La plataforma los pone en una capa propia, sobre las miniaturas.
    pub controls: Option<Controls>,
    /// Cuánto se ve el panel, de 0 (oculto) a 1 (reposo).
    /// El menú siempre vale 1.
    pub opacity: f32,
    /// Escala del panel, con origen en `origin`. Los paneles animados
    /// crecen desde el icono; el menú siempre vale 1.
    pub scale: f32,
    /// Punto que queda fijo al escalar, en unidades lógicas
    /// (centro inferior del panel, sobre el icono).
    pub origin: (f32, f32),
}
