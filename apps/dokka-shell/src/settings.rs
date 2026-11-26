//! Ventana de Preferencias: abrirla, traerla al frente, moverla y dibujarla.
//!
//! La isla decide la sección y qué hace cada clic; aquí solo se conecta con la
//! ventana: el semáforo rojo oculta (Dokka sigue en marcha), el amarillo
//! minimiza y la franja superior arrastra.

use std::error::Error;
use std::sync::Arc;

use dokka_settings::{SettingsAction, drag_zone, settings_scene};
use dokka_theme::settings as measures;
use dokka_win_surface::{Surface, SurfaceEvent};

use crate::dock_session::DockSession;

/// Abre la ventana o la trae al frente si ya estaba abierta.
pub(crate) fn open_or_focus_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    session.settings.open();
    surface.settings_place(measures::WIDTH, measures::HEIGHT)?;
    surface.settings_show()?;
    Ok(())
}

/// Oculta la ventana; Dokka sigue en marcha.
pub(crate) fn close_settings(session: &mut DockSession, surface: &mut Surface) {
    session.settings.close();
    session.settings_drag = None;
    surface.settings_hide();
}

/// Botón abajo: si es en la zona arrastrable, empieza el arrastre.
pub(crate) fn press_over_settings(session: &mut DockSession, x: f32, y: f32) {
    if drag_zone(x, y) {
        session.settings_drag = Some([x, y]);
    }
}

/// Movimiento: arrastra la ventana o resalta la fila bajo el cursor.
pub(crate) fn move_over_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    if let Some([grab_x, grab_y]) = session.settings_drag {
        surface.settings_drag_by(x - grab_x, y - grab_y)?;
        session.settings_drag = Some([x, y]);
        return Ok(());
    }
    session.settings.pointer_moved(x, y);
    Ok(())
}

/// El cursor sale: se apaga el resaltado y se suelta el arrastre.
pub(crate) fn leave_settings(session: &mut DockSession) {
    session.settings_drag = None;
    session.settings.pointer_left();
}

/// Clic: cerrar, minimizar o elegir sección.
pub(crate) fn click_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    session.settings_drag = None;
    match session.settings.click(x, y) {
        SettingsAction::Close => close_settings(session, surface),
        SettingsAction::Minimize => surface.settings_minimize(),
        SettingsAction::Stay => {}
    }
    Ok(())
}

/// Atiende un evento de la ventana si es suyo. Devuelve si lo consumió.
pub(crate) fn handle_settings_event(
    session: &mut DockSession,
    surface: &mut Surface,
    event: SurfaceEvent,
) -> Result<bool, Box<dyn Error>> {
    match event {
        SurfaceEvent::SettingsMoved { x, y } => {
            move_over_settings(session, surface, x, y)?;
        }
        SurfaceEvent::SettingsLeft => leave_settings(session),
        SurfaceEvent::SettingsPressed { x, y } => press_over_settings(session, x, y),
        SurfaceEvent::SettingsClicked { x, y } => click_settings(session, surface, x, y)?,
        _ => return Ok(false),
    }
    Ok(true)
}

/// Dibuja la ventana en su propio árbol visual si está abierta.
pub(crate) fn render_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if !session.settings.is_open() {
        return Ok(());
    }
    surface.settings_place(measures::WIDTH, measures::HEIGHT)?;
    let theme = Arc::clone(&session.theme);
    let scene = settings_scene(&theme, &mut session.settings);
    session.renderer.draw_settings(&scene)?;
    Ok(())
}
