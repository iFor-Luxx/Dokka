use crate::{Divider, Dot, Icon, Label, Overlay, Panel, Shadow, Text, Thumbnail};

/// Todo lo que muestra una superficie.
#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub panel: Panel,
    /// Sombra suave bajo el fondo, con su forma redondeada. Solo Preferencias
    /// la pide; el resto vale `None` y se ve igual que hoy.
    pub panel_shadow: Option<Shadow>,
    /// Paneles extra sobre el fondo y bajo el resaltado, como la casilla de búsqueda.
    pub panels: Vec<Panel>,
    /// Paneles planos (tinte y borde, sin cristal) sobre el cristal de la misma
    /// ventana, como el buscador, la selección y el hover de Preferencias: el
    /// cristal muestrearía el fondo de pantalla e ignoraría lo que hay debajo
    /// dentro de la ventana. El dock, el Launchpad y los paneles no los usan.
    pub flat_panels: Vec<Panel>,
    pub icons: Vec<Icon>,
    pub dots: Vec<Dot>,
    pub dividers: Vec<Divider>,
    pub thumbnails: Vec<Thumbnail>,
    pub labels: Vec<Label>,
    /// Textos sueltos, como los nombres bajo los iconos del Launchpad.
    pub texts: Vec<Text>,
    /// Resaltado suave detrás del elemento bajo el cursor.
    pub highlight: Option<Panel>,
    pub overlay: Option<Overlay>,
    /// Segundo panel del menú (submenú), sobre el principal y sin punta.
    pub submenu: Option<Overlay>,
    /// Cuánto se ve toda la escena, de 0 (oculta) a 1 (reposo). El dock siempre vale 1.
    pub opacity: f32,
    /// Escala del contenido sobre el fondo, con origen en el centro del fondo.
    /// El fondo solo se desvanece; el dock siempre vale 1.
    pub scale: f32,
    /// Desenfoque gaussiano del contenido en unidades lógicas, de 0 (nítido)
    /// al máximo del tema. Solo el Launchpad lo usa durante el zoom; el resto
    /// vale 0. No reconstruye capas: la plataforma lo aplica sobre la escena.
    pub blur: f32,
}
