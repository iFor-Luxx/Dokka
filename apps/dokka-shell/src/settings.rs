//! Ventana de Preferencias: abrirla, traerla al frente, moverla y dibujarla.
//!
//! La isla decide la sección y qué hace cada clic; aquí solo se conecta con la
//! ventana: el semáforo rojo oculta (Dokka sigue en marcha), el amarillo
//! minimiza y la franja superior arrastra con el movimiento nativo de Windows.

use std::error::Error;
use std::sync::Arc;

use dokka_settings::{SettingsAction, drag_zone, settings_scene};
use dokka_theme::settings as measures;
use dokka_win_surface::{Surface, SurfaceEvent};

use crate::dock_session::DockSession;

/// Abre la ventana o la trae al frente si ya estaba abierta.
/// Si estaba minimizada, `SW_SHOWNORMAL` la restaura.
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
    session.settings_pressed = false;
    // La próxima apertura debe dibujar de nuevo: el contenido pudo quedar
    // oculto por el cierre desde la X, Alt+F4 o la barra de tareas.
    session.settings_painted = None;
    surface.settings_hide();
}

/// Botón abajo: si es en la zona arrastrable, Windows mueve la ventana hasta
/// soltar el botón (bucle modal nativo, con captura incluida). El botón que
/// termina el arrastre se lo queda Windows, así que se marca como soltado.
pub(crate) fn press_over_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) {
    session.settings_pressed = true;
    if drag_zone(x, y) {
        surface.settings_begin_drag();
        session.settings_pressed = false;
    }
}

/// Movimiento: solo resalta la fila bajo el cursor; mover ya lo hace Windows.
pub(crate) fn move_over_settings(session: &mut DockSession, x: f32, y: f32) {
    session.settings.pointer_moved(x, y);
}

/// El cursor sale: se apaga el resaltado y se olvida el botón a medias.
pub(crate) fn leave_settings(session: &mut DockSession) {
    session.settings_pressed = false;
    session.settings.pointer_left();
}

/// Clic: cerrar, minimizar o elegir sección.
pub(crate) fn click_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
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
        SurfaceEvent::SettingsMoved { x, y } => move_over_settings(session, x, y),
        SurfaceEvent::SettingsLeft => leave_settings(session),
        SurfaceEvent::SettingsPressed { x, y } => press_over_settings(session, surface, x, y),
        SurfaceEvent::SettingsClicked { x, y } => {
            // Sin botón abajo previo no hay clic (p. ej. soltar dentro lo
            // pulsado fuera); el arrastre ya marcó el suyo como soltado.
            if std::mem::take(&mut session.settings_pressed) {
                click_settings(session, surface, x, y)?;
            }
        }
        // Cierre desde la X, Alt+F4 o la barra de tareas: la ventana ya se
        // ocultó sola; aquí se cierra la isla para poder reabrirla bien.
        SurfaceEvent::SettingsDismissed => close_settings(session, surface),
        _ => return Ok(false),
    }
    Ok(true)
}

/// Dibuja la ventana en su propio árbol visual si está abierta.
/// Si la sección y el resaltado no cambiaron, no se repinta: redibujar el
/// cristal en cada evento (pasar el ratón, hacer clic, arrastrar) lo hacía
/// parpadear. La isla reutiliza los mismos dibujos y las capas solo repintan
/// lo que cambia, así que basta con dibujar al abrir, elegir o resaltar.
pub(crate) fn render_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if !session.settings.is_open() {
        return Ok(());
    }
    let painted = (session.settings.selected(), session.settings.hovered());
    if session.settings_painted == Some(painted) {
        return Ok(());
    }
    surface.settings_place(measures::WIDTH, measures::HEIGHT)?;
    let theme = Arc::clone(&session.theme);
    let scene = settings_scene(&theme, &mut session.settings);
    session.renderer.draw_settings(&scene)?;
    session.settings_painted = Some(painted);
    Ok(())
}
