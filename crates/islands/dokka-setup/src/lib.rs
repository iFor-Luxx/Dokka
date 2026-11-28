//! Isla del instalador propio: estado, lógica, vista y eventos.

mod events;
mod logic;
mod motion;
mod pointer;
mod state;
mod view;

pub use events::SetupAction;
pub use logic::{Control, WorkStep, step_text};
pub use state::{PaintedF32, PaintedSetup, Screen, Setup};
pub use view::setup_scene;
