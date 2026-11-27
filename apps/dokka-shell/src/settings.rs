//! Ventana de Preferencias: abrirla, traerla al frente, moverla y dibujarla.
//!
//! La isla decide la sección y qué hace cada clic; aquí solo se conecta con la
//! ventana: el semáforo rojo oculta (Dokka sigue en marcha), el amarillo
//! minimiza y la franja superior arrastra con el movimiento nativo de Windows.
//! Los deslizadores se arrastran con el ratón capturado: la isla redibuja a
//! cada movimiento y solo al soltar emite `Changed` o `TaskbarAuto`.

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
    session
        .settings
        .set_values(session.preferences, session.taskbar_auto());
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

/// Botón abajo: si agarra un deslizador, la isla lo sigue hasta soltar; si es
/// en la zona arrastrable, Windows mueve la ventana hasta soltar el botón
/// (bucle modal nativo, con captura incluida). El botón que termina el
/// arrastre se lo queda Windows, así que se marca como soltado.
pub(crate) fn press_over_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) {
    session.settings_pressed = true;
    if session.settings.press(x, y) {
        return;
    }
    if drag_zone(x, y, session.settings.selected()) {
        surface.settings_begin_drag();
        session.settings_pressed = false;
    }
}

/// Movimiento: resalta lo que hay bajo el cursor y sigue el deslizador
/// agarrado; mover la ventana ya lo hace Windows.
pub(crate) fn move_over_settings(session: &mut DockSession, x: f32, y: f32) {
    if session.settings_pressed {
        session.settings.drag_to(x, y);
    } else {
        session.settings.pointer_moved(x, y);
    }
}

/// El cursor sale: se apaga el resaltado y se olvida el botón a medias, salvo
/// que haya un deslizador agarrado (con captura sigue llegando el movimiento).
pub(crate) fn leave_settings(session: &mut DockSession) {
    session.settings_pressed = false;
    session.settings.pointer_left();
}

/// Suelta el botón: si había un deslizador agarrado, guarda una sola vez; si
/// no, resuelve el clic (cerrar, minimizar, foco, píldora, sección o control).
pub(crate) fn click_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    match session.settings.release(x, y) {
        SettingsAction::Close => close_settings(session, surface),
        SettingsAction::Minimize => surface.settings_minimize(),
        SettingsAction::Stay => {}
        SettingsAction::Changed(preferences) => {
            crate::apply_preferences::apply_preferences(session, surface, preferences);
        }
        SettingsAction::TaskbarAuto(auto) => {
            crate::apply_preferences::set_taskbar_auto(session, surface, auto);
        }
    }
    Ok(())
}

/// Foco ganado o perdido: sin foco los semáforos se ven grises.
pub(crate) fn focus_over_settings(session: &mut DockSession, focused: bool) {
    if session.settings.is_open() {
        session.settings.set_window_focused(focused);
    }
}

/// Letra escrita con la casilla enfocada: filtra las filas.
pub(crate) fn type_over_settings(session: &mut DockSession, character: char) {
    if session.settings.is_open() {
        session.settings.type_character(character);
    }
}

/// Retroceso con la casilla enfocada: borra la última letra.
pub(crate) fn erase_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.erase_character();
    }
}

/// Enter con búsqueda: elige la primera coincidencia.
pub(crate) fn confirm_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.confirm_search();
    }
}

/// Esc: limpia lo escrito y quita el foco.
pub(crate) fn escape_over_settings(session: &mut DockSession) {
    if session.settings.is_open() {
        session.settings.escape_search();
    }
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
            // pulsado fuera); el arrastre nativo ya marcó el suyo como soltado.
            if std::mem::take(&mut session.settings_pressed) {
                click_settings(session, surface, x, y)?;
            }
        }
        // Cierre desde la X, Alt+F4 o la barra de tareas: la ventana ya se
        // ocultó sola; aquí se cierra la isla para poder reabrirla bien.
        SurfaceEvent::SettingsDismissed => close_settings(session, surface),
        SurfaceEvent::SettingsFocused => focus_over_settings(session, true),
        SurfaceEvent::SettingsBlurred => focus_over_settings(session, false),
        SurfaceEvent::SettingsTyped { character } => type_over_settings(session, character),
        SurfaceEvent::SettingsBackspace => erase_over_settings(session),
        SurfaceEvent::SettingsEnter => confirm_over_settings(session),
        SurfaceEvent::SettingsEscape => escape_over_settings(session),
        _ => return Ok(false),
    }
    Ok(true)
}

/// Dibuja la ventana en su propio árbol visual si está abierta.
/// Si nada de lo que se ve cambió, no se repinta: redibujar el cristal en
/// cada evento (pasar el ratón, hacer clic, arrastrar) lo hacía parpadear.
/// La isla reutiliza los mismos dibujos y las capas solo repintan lo que
/// cambia, así que basta con dibujar al abrir, elegir, resaltar, enfocar,
/// escribir, cambiar el foco o mover un deslizador.
pub(crate) fn render_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if !session.settings.is_open() {
        return Ok(());
    }
    let painted = session.settings.painted();
    if session.settings_painted.as_ref() == Some(&painted) {
        return Ok(());
    }
    surface.settings_place(measures::WIDTH, measures::HEIGHT)?;
    let theme = Arc::clone(&session.theme);
    let scene = settings_scene(&theme, &mut session.settings);
    session.renderer.draw_settings(&scene)?;
    session.settings_painted = Some(painted);
    Ok(())
}
