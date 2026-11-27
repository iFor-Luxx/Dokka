//! Mes completo con su píldora de navegación: claro en el dock y cristal
//! oscuro en la barra (`dark`), sin duplicar el trazado.

mod month;
mod navigation;
mod props;
mod view;

pub use month::calendar_month;
pub use navigation::{calendar_hovered, calendar_pill_props, calendar_step};
pub use props::CalendarMonth;
pub use view::{calendar_nav_pill, calendar_panel, calendar_size};
