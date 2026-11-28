use crate::{Color, Divider, Icon, Panel, Rect, Text};

/// Franja difuminada bajo la cabecera de Preferencias, estilo Tahoe.
///
/// Describe el rectángulo que se desenfoca (zona de contenido, del borde
/// superior hasta un poco por debajo del título), el tinte del fondo en
/// degradado, el radio del desenfoque y su intensidad (0 arriba del todo,
/// 1 al desplazar). Trae además las piezas que van por encima, nítidas:
/// la píldora (cristal y resaltado), sus líneas, sus iconos y el título.
/// La isla no sabe de Composition; la plataforma la dibuja entre el
/// contenido y esas piezas.
#[derive(Debug, Clone, PartialEq)]
pub struct HeaderVeil {
    pub bounds: Rect,
    pub tint: Color,
    pub blur_radius: f32,
    /// De 0 (arriba del todo, invisible) a 1 (desplazado, plena).
    pub strength: f32,
    /// Cristal de la píldora, sobre el difuminado.
    pub panels: Vec<Panel>,
    /// Resaltado de la píldora, plano sobre el difuminado.
    pub flats: Vec<Panel>,
    pub dividers: Vec<Divider>,
    pub icons: Vec<Icon>,
    pub texts: Vec<Text>,
}
