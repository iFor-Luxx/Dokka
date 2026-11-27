use dokka_theme::Theme;

/// El deslizador no necesita dibujos propios: la barra y el relleno son
/// planos, sin tirador. Existe para que la isla lo cree una vez
/// como el resto de iconos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliderIcons;

impl SliderIcons {
    pub fn new(_theme: &Theme) -> Self {
        Self
    }
}
