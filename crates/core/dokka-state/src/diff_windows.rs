use dokka_events::Event;
use dokka_model::OpenWindow;

use crate::DockState;

impl DockState {
    /// Eventos que llevan el estado hasta la lista de ventanas observada ahora.
    pub fn changes_for(&self, observed: &[OpenWindow]) -> Vec<Event> {
        let closed = self.apps.iter().flat_map(|app| {
            app.windows
                .iter()
                .filter(|known| {
                    !observed
                        .iter()
                        .any(|open| open.app.id == app.info.id && open.window.id == known.id)
                })
                .map(|known| Event::WindowClosed(known.id))
        });

        let opened = observed
            .iter()
            .filter(|open| {
                !self
                    .apps
                    .iter()
                    .any(|app| app.info.id == open.app.id && app.windows.contains(&open.window))
            })
            .map(|open| Event::WindowOpened(open.clone()));

        // Primero los cierres: Windows puede reutilizar un identificador.
        closed.chain(opened).collect()
    }
}
