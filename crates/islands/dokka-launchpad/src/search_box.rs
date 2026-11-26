//! Texto, foco y parpadeo de la casilla de búsqueda.
//!
//! Sin foco muestra lupa y "Buscar" a la izquierda; con foco aparece el cursor
//! vertical tras el texto y el marcador tenue a su derecha. Escribir enfoca;
//! Esc limpia y quita el foco, y con la casilla vacía y sin foco cierra.

use dokka_theme::launchpad as measures;

use crate::Launchpad;
use crate::LaunchpadAction;
use crate::motion::PageMotion;

/// Lo que pide Esc: seguir abierto o cerrar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EscapeOutcome {
    Stay,
    Close,
}

/// Cuadro de búsqueda sin saber qué es el Launchpad.
#[derive(Debug, Default)]
pub(crate) struct SearchBox {
    query: String,
    focused: bool,
    cursor_visible: bool,
}

impl SearchBox {
    pub(crate) fn reset(&mut self) {
        self.query.clear();
        self.focused = false;
        self.cursor_visible = false;
    }

    pub(crate) fn query(&self) -> &str {
        &self.query
    }

    pub(crate) fn is_focused(&self) -> bool {
        self.focused
    }

    pub(crate) fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Clic en la casilla: enfoca y muestra el cursor, sin cerrar nada.
    pub(crate) fn focus(&mut self) {
        self.focused = true;
        self.cursor_visible = true;
    }

    /// Añade lo escrito y enfoca, para que escribir sin clicar funcione.
    pub(crate) fn type_character(&mut self, character: char) {
        if character.is_control() {
            return;
        }
        if self.query.chars().count() >= measures::SEARCH_MAX_CHARACTERS {
            return;
        }
        self.query.push(character);
        self.focus();
    }

    /// Borra la última letra, manteniendo el foco.
    pub(crate) fn erase_character(&mut self) {
        self.query.pop();
    }

    /// Esc limpia lo escrito y quita el foco; con la casilla vacía y sin foco, cierra.
    pub(crate) fn escape(&mut self) -> EscapeOutcome {
        if !self.query.is_empty() {
            self.query.clear();
            self.focused = false;
            self.cursor_visible = false;
            return EscapeOutcome::Stay;
        }
        if self.focused {
            self.focused = false;
            self.cursor_visible = false;
            return EscapeOutcome::Stay;
        }
        EscapeOutcome::Close
    }

    /// Alterna el parpadeo del cursor con el aviso de cada segundo.
    pub(crate) fn tick(&mut self) {
        if self.focused {
            self.cursor_visible = !self.cursor_visible;
        } else {
            self.cursor_visible = false;
        }
    }
}

impl Launchpad {
    /// Añade lo escrito, enfoca y vuelve a la primera página.
    pub fn type_character(&mut self, character: char) {
        self.search.type_character(character);
        self.page = 0;
        self.motion = PageMotion::new();
        self.highlighted = None;
    }

    /// Borra la última letra y vuelve a la primera página.
    pub fn erase_character(&mut self) {
        self.search.erase_character();
        self.page = 0;
        self.motion = PageMotion::new();
        self.highlighted = None;
    }

    /// Alterna el parpadeo del cursor con el aviso de cada segundo.
    pub fn tick(&mut self) {
        self.search.tick();
    }

    /// Enter abre la primera coincidencia; sin texto no hace nada.
    pub fn confirm(&self) -> LaunchpadAction {
        if self.query().is_empty() {
            return LaunchpadAction::Stay;
        }
        match self.first_match() {
            Some(app) => LaunchpadAction::Open(app),
            None => LaunchpadAction::Stay,
        }
    }

    /// Esc limpia y quita el foco; con la casilla vacía y sin foco, cierra.
    pub fn escape(&mut self) -> LaunchpadAction {
        match self.search.escape() {
            EscapeOutcome::Close => LaunchpadAction::Close,
            EscapeOutcome::Stay => {
                self.page = 0;
                self.motion = PageMotion::new();
                self.highlighted = None;
                LaunchpadAction::Stay
            }
        }
    }
}
