//! Ventana de Preferencias: abrirla, traerla al frente, moverla y dibujarla.
//!
//! La isla decide la sección y qué hace cada clic; aquí solo se conecta con la
//! ventana: el semáforo rojo cierra y limpia la navegación (volver a abrir
//! empieza en General), el amarillo minimiza al dock propio y el verde alterna
//! el tamaño. Los deslizadores se arrastran con el ratón capturado: la isla
//! redibuja a cada movimiento y solo al soltar emite `Changed` o `TaskbarAuto`.

use std::error::Error;
use std::sync::Arc;

use dokka_settings::{GeneralPage, SettingsAction, drag_zone, settings_scene};
use dokka_win_surface::{Surface, SurfaceEvent};

use crate::dock_session::DockSession;
use crate::settings_keyboard::{
    confirm_over_settings, erase_over_settings, escape_over_settings, focus_over_settings,
    type_over_settings,
};

/// Abre la ventana o la trae al frente si ya estaba abierta.
/// Si estaba minimizada, `SW_SHOWNORMAL` la restaura.
pub(crate) fn open_or_focus_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    session
        .settings
        .set_values(session.preferences, session.taskbar_auto());
    session
        .settings
        .set_dokka_version(env!("CARGO_PKG_VERSION").to_owned());
    session.settings.open();
    // Primera apertura: centra el contenido una vez con el aire alrededor;
    // luego conserva su sitio al moverla. El zoom aplica su tamaño aparte.
    surface.settings_place(
        session.settings.window_width(),
        session.settings.window_height(),
        dokka_theme::settings::SHADOW_MARGIN,
    )?;
    surface.settings_show()?;
    // Restaurar quita la miniatura del dock: el observador ignora el proceso
    // propio, así que se sincroniza aquí en vez de esperar un aviso externo.
    let _ = session.sync();
    Ok(())
}

/// Oculta la ventana; Dokka sigue en marcha. Cierra y limpia la navegación:
/// al volver a abrir empieza en General, sin historial, búsqueda ni foco. Los
/// ajustes guardados no se tocan. Vuelve además al tamaño normal.
pub(crate) fn close_settings(session: &mut DockSession, surface: &mut Surface) {
    crate::settings_system::cancel_storage(session);
    session.settings.close();
    session.settings_pressed = false;
    // La próxima apertura debe dibujar de nuevo: el contenido pudo quedar
    // oculto por el cierre desde la X, Alt+F4 o la barra de tareas.
    session.settings_painted = None;
    surface.settings_hide();
    // Vuelve al tamaño normal de contenido ya oculta, sin parpadeo.
    let _ = surface.settings_apply_size(
        dokka_theme::settings::WIDTH,
        dokka_theme::settings::HEIGHT,
        dokka_theme::settings::SHADOW_MARGIN,
    );
    // Ocultar quita la miniatura del dock: el observador ignora el proceso
    // propio, así que se sincroniza aquí.
    let _ = session.sync();
}

/// Alterna el tamaño normal y el mayor, recentrado sin salirse de la pantalla.
/// La barra lateral conserva su ancho; el contenido se ensancha solo.
pub(crate) fn toggle_settings_zoom(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    session.settings.toggle_zoom();
    let width = session.settings.window_width();
    let height = session.settings.window_height();
    surface.settings_apply_size(width, height, dokka_theme::settings::SHADOW_MARGIN)?;
    Ok(())
}

/// Coordenadas de contenido desde las de ventana (con el aire de la sombra).
fn to_content(x: f32, y: f32) -> (f32, f32) {
    let margin = dokka_theme::settings::SHADOW_MARGIN;
    (x - margin, y - margin)
}

/// `true` si el punto cae en el contenido (fuera está el aire de la sombra).
fn in_content(session: &DockSession, x: f32, y: f32) -> bool {
    let (cx, cy) = to_content(x, y);
    cx >= 0.0
        && cy >= 0.0
        && cx < session.settings.window_width()
        && cy < session.settings.window_height()
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
    // Clic en el aire de la sombra: no hace nada.
    if !in_content(session, x, y) {
        session.settings_pressed = false;
        return;
    }
    let (cx, cy) = to_content(x, y);
    session.settings_pressed = true;
    if session.settings.press(cx, cy) {
        return;
    }
    let disks = session
        .settings
        .storage_breakdown()
        .map(|known| known.other_disks.len())
        .unwrap_or(0);
    if drag_zone(
        cx,
        cy,
        session.settings.selected(),
        session.settings.content_width(),
        session.settings.general(),
        session.settings.scroll_offset(),
        disks,
    ) {
        surface.settings_begin_drag();
        session.settings_pressed = false;
    }
}

/// Movimiento: resalta lo que hay bajo el cursor y sigue el deslizador
/// agarrado; mover la ventana ya lo hace Windows. En el aire sin botón
/// se apaga el resaltado; con botón se sigue (clamp) para no perder el arrastre.
pub(crate) fn move_over_settings(session: &mut DockSession, x: f32, y: f32) {
    if session.settings_pressed {
        let (cx, cy) = to_content(x, y);
        session.settings.drag_to(cx, cy);
    } else if !in_content(session, x, y) {
        session.settings.pointer_left();
    } else {
        let (cx, cy) = to_content(x, y);
        session.settings.pointer_moved(cx, cy);
    }
}

/// El cursor sale: se apaga el resaltado y se olvida el botón a medias, salvo
/// que haya un deslizador agarrado (con captura sigue llegando el movimiento).
pub(crate) fn leave_settings(session: &mut DockSession) {
    session.settings_pressed = false;
    session.settings.pointer_left();
}

/// Suelta el botón: si había un deslizador agarrado, guarda una sola vez; si
/// no, resuelve el clic (cerrar, minimizar, zoom, foco, píldora, sección o control).
pub(crate) fn click_settings(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
) -> Result<(), Box<dyn Error>> {
    // La suelta llega con captura aunque sea en el aire: se traduce igual y la
    // isla la sujeta al extremo más cercano; un clic simple en el aire nunca
    // llega aquí porque su botón abajo ya se ignoró.
    let (cx, cy) = to_content(x, y);
    match session.settings.release(cx, cy) {
        SettingsAction::Close => close_settings(session, surface),
        SettingsAction::Minimize => {
            surface.settings_minimize();
            // Minimizar crea la miniatura del dock: el observador ignora el
            // proceso propio, así que se sincroniza aquí.
            let _ = session.sync();
        }
        SettingsAction::Zoom => toggle_settings_zoom(session, surface)?,
        SettingsAction::Stay => {}
        SettingsAction::Changed(preferences) => {
            crate::apply_preferences::apply_preferences(session, surface, preferences);
        }
        SettingsAction::TaskbarAuto(auto) => {
            crate::apply_preferences::set_taskbar_auto(session, surface, auto);
        }
        SettingsAction::EnterGeneral(page) => {
            if page == GeneralPage::About {
                crate::settings_system::request_about(session);
            } else if page == GeneralPage::Storage {
                crate::settings_system::request_storage(session);
            }
        }
        SettingsAction::OpenAbout(button) => {
            crate::settings_system::open_about_page(session, button);
        }
        SettingsAction::EmptyTrash => {
            crate::settings_system::empty_trash(session);
        }
    }
    // Volver con la píldora a "Acerca" también pide los datos una vez.
    if session.settings.general() == GeneralPage::About
        && session.settings.system_summary().is_none()
    {
        crate::settings_system::request_about(session);
    }
    // Volver con la píldora a "Almacenamiento" también calcula una vez.
    if session.settings.general() == GeneralPage::Storage
        && session.settings.storage_breakdown().is_none()
    {
        crate::settings_system::request_storage(session);
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
        SurfaceEvent::SettingsWheel { delta } => {
            crate::settings_system::scroll_settings(session, delta)
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// Dibuja la ventana en su propio árbol visual si está abierta.
/// Si nada de lo que se ve cambió, no se repinta: redibujar el cristal en
/// cada evento (pasar el ratón, hacer clic, arrastrar) lo hacía parpadear.
/// La isla reutiliza los mismos dibujos y las capas solo repintan lo que
/// cambia, así que basta con dibujar al abrir, elegir, resaltar, enfocar,
/// escribir, cambiar el foco, el zoom o mover un deslizador.
pub(crate) fn render_settings(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    if !session.settings.is_open() {
        return Ok(());
    }
    // El hilo ya leyó el equipo: pasa el último valor a la isla para que
    // repinte "Acerca" sin pedir en bucle.
    crate::settings_system::poll_about(session);
    // El hilo del desglose avanza: si se está en "Almacenamiento" se pasa lo
    // nuevo; si no, se detiene al salir del apartado.
    if session.settings.general() == GeneralPage::Storage {
        crate::settings_system::poll_storage(session);
    } else {
        crate::settings_system::cancel_storage(session);
    }
    let painted = session.settings.painted();
    if session.settings_painted.as_ref() == Some(&painted) {
        return Ok(());
    }
    // El tamaño lo aplican abrir y el zoom; aquí solo se asegura la primera
    // colocación. No se recentra al pasar el cursor: la ventana conserva su sitio.
    surface.settings_place(
        session.settings.window_width(),
        session.settings.window_height(),
        dokka_theme::settings::SHADOW_MARGIN,
    )?;
    let theme = Arc::clone(&session.theme);
    let scene = settings_scene(&theme, &mut session.settings);
    session.renderer.draw_settings(&scene)?;
    session.settings_painted = Some(painted);
    Ok(())
}
