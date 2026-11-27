//! Arrastre para reordenar la barra: pulsar, umbral y hueco.
//!
//! Se arrastra desde cualquier punto de la fila salvo el interruptor. Un clic
//! simple no mueve nada: hasta pasar unos 4 px no hay arrastre. La fila
//! agarrada sigue al cursor en vertical sin salirse de la tarjeta; las demás
//! se apartan dejando el hueco donde caerá. Al soltar, una sola vez y solo si
//! cambió el orden.

use dokka_model::MenubarItem;

/// Arrastre de una fila a medias: dónde se agarró y dónde caería.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MenubarDrag {
    /// Punto donde se pulsó, para el umbral de 4 px.
    press_x: f32,
    press_y: f32,
    /// Fila agarrada en el orden base.
    from: usize,
    /// Orden al pulsar, sin el arrastre aplicado.
    base: [MenubarItem; 7],
    /// `true` cuando ya pasó el umbral y la fila sigue al cursor.
    active: bool,
    /// Cursor actual en vertical, para la fila flotante.
    current_y: f32,
    /// Hueco donde caería al soltar.
    target: usize,
}

impl MenubarDrag {
    pub(crate) fn pressed(x: f32, y: f32, from: usize, base: [MenubarItem; 7]) -> Self {
        Self {
            press_x: x,
            press_y: y,
            from,
            base,
            active: false,
            current_y: y,
            target: from,
        }
    }

    /// Avanza el arrastre hasta este punto; devuelve si ya pasó el umbral.
    pub(crate) fn moved(&mut self, x: f32, y: f32, threshold: f32, target: usize) -> bool {
        self.current_y = y;
        if !self.active {
            let moved = ((x - self.press_x).powi(2) + (y - self.press_y).powi(2)).sqrt();
            if moved <= threshold {
                return false;
            }
            self.active = true;
        }
        self.target = target;
        true
    }

    pub(crate) fn is_active(self) -> bool {
        self.active
    }

    pub(crate) fn from(self) -> usize {
        self.from
    }

    pub(crate) fn target(self) -> usize {
        self.target
    }

    pub(crate) fn current_y(self) -> f32 {
        self.current_y
    }

    pub(crate) fn item(self) -> MenubarItem {
        self.base[self.from.min(6)]
    }

    /// Orden mostrado con el hueco actual, si ya pasó el umbral.
    pub(crate) fn display(self) -> [MenubarItem; 7] {
        if !self.active {
            return self.base;
        }
        super::logic::reordered(&self.base, self.from, self.target)
    }
}
