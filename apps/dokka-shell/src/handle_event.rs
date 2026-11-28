//! Reparte cada evento de la ventana a la pieza que corresponde.

use std::error::Error;

use dokka_model::WindowId;
use dokka_previews::PreviewClick;
use dokka_win_surface::{Surface, SurfaceEvent};

use crate::context_menu::{open_context_menu, poll_app_picker, run_menu_action};
use crate::dock_session::DockSession;
use crate::floating_panels::preview_request;
use crate::launchpad::{
    begin_close_launchpad, click_launchpad, confirm_over_launchpad, erase_over_launchpad,
    escape_over_launchpad, move_over_launchpad, page_back_launchpad, page_forward_launchpad,
    scroll_launchpad, type_over_launchpad,
};
use crate::media::send_media;
use crate::perform_click::perform_click;
use crate::preview_safety;
use crate::settings::handle_settings_event;

pub(crate) fn handle_event(
    session: &mut DockSession,
    surface: &mut Surface,
    event: SurfaceEvent,
) -> Result<(), Box<dyn Error>> {
    if handle_settings_event(session, surface, event)? {
        return preview_safety::render_and_watch(session, surface);
    }
    if crate::menubar::handle_menubar_event(session, surface, event)? {
        return preview_safety::render_and_watch(session, surface);
    }
    match event {
        // Los eventos de Preferencias ya se atendieron arriba.
        SurfaceEvent::SettingsMoved { .. }
        | SurfaceEvent::SettingsLeft
        | SurfaceEvent::SettingsPressed { .. }
        | SurfaceEvent::SettingsClicked { .. }
        | SurfaceEvent::SettingsDismissed
        | SurfaceEvent::SettingsFocused
        | SurfaceEvent::SettingsBlurred
        | SurfaceEvent::SettingsTyped { .. }
        | SurfaceEvent::SettingsBackspace
        | SurfaceEvent::SettingsEnter
        | SurfaceEvent::SettingsEscape
        | SurfaceEvent::SettingsWheel { .. }
        | SurfaceEvent::MenubarMoved { .. }
        | SurfaceEvent::MenubarLeft
        | SurfaceEvent::MenubarPressed { .. }
        | SurfaceEvent::MenubarClicked { .. }
        | SurfaceEvent::MenubarTyped { .. }
        | SurfaceEvent::MenubarBackspace
        | SurfaceEvent::MenubarWheel { .. }
        | SurfaceEvent::MenubarEnter
        | SurfaceEvent::MenubarUp
        | SurfaceEvent::MenubarDown
        | SurfaceEvent::MenubarRightClicked { .. }
        | SurfaceEvent::MenubarDismissed => {}
        SurfaceEvent::PointerMoved { x, y } => {
            session.dock.note_cursor_inside(true);
            if session.launchpad.is_open() {
                session.dock.pointer_moved(x, y, session.state.apps());
            } else if session.menubar.is_menu_open()
                || session.menubar.is_calendar_open()
                || session.menubar.is_tray_open()
            {
                // Con la barra desplegada no se abren previas ni se mueve el dock.
                session.previews.close();
            } else if session.dock.is_menu_open() {
                session.dock.move_over_menu(x, y);
            } else if session.previews.contains(x, y) {
                // Sobre el panel el dock mantiene la lupa tal como estaba.
                session.previews.pointer_moved(x, y, None);
            } else {
                session.dock.pointer_moved(x, y, session.state.apps());
                let request = preview_request(session, x, y);
                session.previews.pointer_moved(x, y, request);
            }
        }
        SurfaceEvent::PointerLeft => {
            session.dock.note_cursor_inside(false);
            if session.launchpad.is_open() {
                session.dock.pointer_left();
            } else if !session.dock.is_menu_open() {
                session.dock.pointer_left();
                session.previews.pointer_left();
            }
        }
        SurfaceEvent::LaunchpadMoved { x, y } => {
            // Sin cambio de resaltado no se redibuja: el parpadeo del cursor
            // llega por `Tick` y los iconos ya están en caché tras abrir.
            if !move_over_launchpad(session, surface, x, y) {
                return Ok(());
            }
        }
        SurfaceEvent::LaunchpadLeft => {
            if !session.launchpad.pointer_left() {
                return Ok(());
            }
        }
        SurfaceEvent::LaunchpadClicked { x, y } => {
            click_launchpad(session, surface, x, y)?;
        }
        SurfaceEvent::LaunchpadDismissed => {
            if session.launchpad.is_open() && !session.launchpad.is_closing() {
                begin_close_launchpad(session, surface);
            }
        }
        SurfaceEvent::LaunchpadTyped { character } => {
            if session.launchpad.is_open() {
                type_over_launchpad(session, character);
            }
        }
        SurfaceEvent::LaunchpadBackspace => {
            if session.launchpad.is_open() {
                erase_over_launchpad(session);
            }
        }
        SurfaceEvent::LaunchpadEnter => {
            if session.launchpad.is_open() {
                confirm_over_launchpad(session, surface)?;
            }
        }
        SurfaceEvent::LaunchpadEscape => {
            if session.launchpad.is_open() {
                escape_over_launchpad(session, surface)?;
            }
        }
        SurfaceEvent::LaunchpadWheel { delta } => {
            if session.launchpad.is_open() {
                scroll_launchpad(session, surface, delta);
            }
        }
        SurfaceEvent::LaunchpadPrevPage => {
            if session.launchpad.is_open() {
                page_back_launchpad(session, surface);
            }
        }
        SurfaceEvent::LaunchpadNextPage => {
            if session.launchpad.is_open() {
                page_forward_launchpad(session, surface);
            }
        }
        SurfaceEvent::Frame { seconds } => {
            crate::handle_frame::handle_frame(session, surface, seconds)?;
        }
        SurfaceEvent::CursorCheck => {
            if !session.launchpad.is_open() {
                preview_safety::check_preview_cursor(session, surface);
            }
        }
        SurfaceEvent::Refresh => {
            // Avisa también cuando Windows saca su barra por su cuenta.
            crate::taskbar_mode::keep_taskbar_hidden(session);
            let changed = session.sync()?;
            // Otra ventana al frente también avisa, aunque la lista siga igual.
            let foreground_changed = session.poll_foreground_changed();
            let occluded_changed = session.refresh_occlusion();
            if !changed && !foreground_changed && !occluded_changed {
                return Ok(());
            }
            if !session.launchpad.is_open() {
                preview_safety::close_if_cursor_away(session, surface);
            }
            if !changed && !occluded_changed {
                return preview_safety::render_and_watch(session, surface);
            }
            if session.dock.is_menu_open() {
                session.dock.close_menu();
                surface.menu_closed();
            }
            session.resize(surface)?;
        }
        SurfaceEvent::Tick => {
            crate::taskbar_mode::keep_taskbar_hidden(session);
            session.update_clock_and_trash();
            preview_safety::check_preview_cursor(session, surface);
            if session.refresh_occlusion() {
                session.resize(surface)?;
            }
            if session.launchpad.is_open() {
                session.launchpad.tick();
            }
            poll_app_picker(session, surface)?;
        }
        SurfaceEvent::Dismissed => {
            if session.dock.is_menu_open() {
                session.dock.close_menu();
                surface.menu_closed();
            }
        }
        SurfaceEvent::Clicked { x, y, with_shift } => {
            click(session, surface, x, y, with_shift)?;
        }
        SurfaceEvent::RightClicked { x, y } => {
            if session.launchpad.is_open() {
                begin_close_launchpad(session, surface);
            } else {
                session.previews.close();
                open_context_menu(session, surface, x, y)?;
            }
        }
    }
    preview_safety::render_and_watch(session, surface)
}

/// Los paneles están por encima: tienen prioridad sobre el dock.
fn click(
    session: &mut DockSession,
    surface: &mut Surface,
    x: f32,
    y: f32,
    with_shift: bool,
) -> Result<(), Box<dyn Error>> {
    // Un clic en el dock es fuera de la barra: cierra su menú.
    crate::menubar_dismiss::dismiss_menubar(session, surface);
    if session.dock.is_menu_open() {
        if let Some(action) = session.dock.click_menu(x, y) {
            return run_menu_action(session, surface, action);
        }
        // Un clic fuera del menú sobre el dock solo cierra el menú.
        if !session.dock.menu_contains(x, y) {
            session.dock.close_menu();
            surface.menu_closed();
        }
        return Ok(());
    }
    if session.launchpad.is_open() {
        session.previews.close();
        perform_click(session, surface, x, y, with_shift)?;
        return Ok(());
    }
    match session.previews.click(x, y) {
        Some(PreviewClick::Activate(window)) => {
            crate::window_activation::activate_window(session, surface, WindowId::new(window))?;
        }
        Some(PreviewClick::Close(window)) => {
            crate::preview_actions::close_preview_window(session, surface, window);
        }
        Some(PreviewClick::TogglePin(window)) => {
            crate::preview_actions::toggle_pin_preview_window(session, surface, window);
        }
        Some(PreviewClick::Media(command)) => send_media(session, command),
        Some(PreviewClick::Handled) => {}
        None => {
            session.previews.close();
            perform_click(session, surface, x, y, with_shift)?;
        }
    }
    Ok(())
}
