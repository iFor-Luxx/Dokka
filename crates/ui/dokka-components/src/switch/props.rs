/// Lo que necesita el interruptor, sin saber qué ajusta.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwitchProps {
    /// `true` cuando está encendido (tirador a la derecha).
    pub on: bool,
}
