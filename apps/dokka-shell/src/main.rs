//! Punto de entrada de Dokka: crea las piezas y las conecta.

mod app_menu_actions;
mod apply_preferences;
mod context_menu;
mod dock_fit;
mod dock_session;
mod floating_panels;
mod genie;
mod handle_event;
mod launchpad;
mod media;
mod menubar;
mod menubar_actions;
mod menubar_background;
mod menubar_brightness;
mod menubar_language;
mod menubar_music;
mod menubar_network;
mod menubar_search;
mod menubar_volume;
mod perform_click;
mod pinned_apps;
mod preview_actions;
mod preview_large;
mod preview_safety;
mod session_render;
mod settings;
mod taskbar_mode;

use std::error::Error;

use dokka_theme::typography;

use dokka_win_apps::explorer_app;
use dokka_win_composition::{SceneRenderer, WindowHandles};
use dokka_win_control::set_taskbar_visible;
use dokka_win_surface::{Surface, SurfaceOptions};
use dokka_win_tracking::{ChangeWatcher, WINDOWS_CHANGED_MESSAGE};

use crate::dock_session::DockSession;
use crate::handle_event::handle_event;
use crate::pinned_apps::load_pinned;
use crate::taskbar_mode::load_taskbar_mode;

fn main() -> Result<(), Box<dyn Error>> {
    // Recupera la barra si un cierre anterior la dejó oculta.
    set_taskbar_visible(true);
    // La ventana se crea primero: declara el modo de DPI de todo el proceso.
    let mut surface = Surface::create(SurfaceOptions {
        width: 0.0,
        height: 0.0,
        bottom_margin: 0.0,
        over_taskbar: false,
        position: dokka_model::DockPosition::Bottom,
        hide: 0.0,
    })?;
    surface.settings_place(dokka_theme::settings::WIDTH, dokka_theme::settings::HEIGHT)?;
    // Sin barra superior Dokka sigue funcionando.
    if let Err(error) = crate::menubar::setup_menubar(&mut surface) {
        eprintln!("Dokka no pudo mostrar la barra superior: {error}");
    }
    let renderer = SceneRenderer::attach(
        WindowHandles {
            main: surface.handle(),
            above: surface.above_handle(),
            launchpad: surface.launchpad_handle(),
            preview: surface.preview_handle(),
            settings: surface.settings_handle(),
            menubar: surface.menubar_handle(),
        },
        surface.scale(),
        surface.settings_scale(),
        typography::FONT_FAMILY,
        dokka_icons::LUCIDE_FONT_BYTES,
    )?;
    let explorer = explorer_app();
    let mut session = DockSession::new(renderer, explorer.id.clone());
    // El seguimiento incluye SOLO Preferencias del proceso propio, minimizada.
    session.remember_settings_window(surface.settings_handle());
    // El vuelo pasa tras el cristal del dock.
    session
        .genie
        .set_dock_window(surface.handle().raw().0 as isize);
    session
        .genie
        .set_settings_window(surface.settings_handle().raw().0 as u64);
    let _watcher = ChangeWatcher::start();

    load_pinned(&mut session, explorer);
    if load_taskbar_mode(&mut session) {
        // En automático la barra ya empieza oculta.
        set_taskbar_visible(false);
    }
    crate::apply_preferences::load_preferences(&mut session);
    session.sync()?;
    // Por si un cierre brusco dejo transiciones apagadas, se devuelven a las
    // abiertas ahora.
    {
        let open = session
            .state
            .apps()
            .iter()
            .flat_map(|app| app.windows.iter())
            .map(|window| window.id.value() as isize)
            .collect::<Vec<_>>();
        dokka_win_genie::ensure_clean_startup(&open);
    }
    session.update_clock_and_trash();
    crate::menubar::refresh_foreground(&mut session);
    crate::menubar::refresh_clock(&mut session);
    session.resize(&mut surface)?;
    // La animación propia debe estar apagada antes de la primera minimización.
    crate::genie::refresh(&mut session, &surface);
    crate::session_render::render(&mut session, &mut surface)?;

    println!("Dokka en marcha. Para cerrarlo: Ctrl+C en esta terminal.");
    let result = surface.run(WINDOWS_CHANGED_MESSAGE, |surface, event| {
        if let Err(error) = handle_event(&mut session, surface, event) {
            eprintln!("Dokka no pudo atender un evento: {error}");
        }
        session.is_animating()
    });
    // La barra siempre vuelve, también si `run` terminó con error.
    // El genio devuelve las transiciones que apago (tambien con "Reiniciar").
    session.genie.shutdown();
    surface.menubar_hide();
    set_taskbar_visible(true);
    result?;
    Ok(())
}
