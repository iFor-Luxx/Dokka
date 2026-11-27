//! Dibuja el instante actual en la ventana del dock y en la del Launchpad.

use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use dokka_launchpad::launchpad_scene;
use dokka_model::AppId;
use dokka_scene::{Bitmap, Scene};
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;
use crate::launchpad::launchpad_size;
use crate::media::update_media;
use crate::preview_actions::update_pinned;
use crate::preview_large::{above_window_area, update_large_preview};
use crate::settings::render_settings;

/// Dibuja el instante actual.
pub(crate) fn render(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    update_media(session);
    update_pinned(session);
    let mut scene = dock_scene(session);
    if let Some((flying, opacity)) = session.genie.landing() {
        for thumbnail in &mut scene.thumbnails {
            if thumbnail.window == flying {
                thumbnail.opacity = opacity;
            }
        }
    }
    if session.launchpad.is_open() {
        // Con el Launchpad no hay menú ni vistas previas.
        scene.overlay = None;
        scene.submenu = None;
        scene.labels.clear();
    } else if session.dock.is_menu_open() {
        scene.overlay = session.dock.menu_overlay();
        scene.submenu = session.dock.submenu_overlay();
    } else {
        scene.overlay = session.previews.overlay();
        scene.submenu = None;
    }
    // Con un panel abierto, la etiqueta sobra.
    if scene.overlay.is_some() || scene.submenu.is_some() {
        scene.labels.clear();
    }
    session.renderer.draw(&scene)?;
    let controls = scene.overlay.and_then(|overlay| overlay.controls);
    surface.show_above(controls.as_ref().map(above_window_area))?;
    update_large_preview(session, surface);
    let areas: Vec<[f32; 4]> = if session.launchpad.is_open() {
        [Some(session.dock.active_area(session.state.apps()))]
            .into_iter()
            .flatten()
            .map(|area| [area.x, area.y, area.width, area.height])
            .collect()
    } else {
        [
            Some(session.dock.active_area(session.state.apps())),
            session.dock.menu_area(),
            session.dock.submenu_area(),
            (!session.dock.is_menu_open())
                .then(|| session.previews.area())
                .flatten(),
        ]
        .into_iter()
        .flatten()
        .map(|area| [area.x, area.y, area.width, area.height])
        .collect()
    };
    surface.set_active_areas(&areas);
    if session.launchpad.is_open() {
        render_launchpad(session, surface)?;
    }
    render_settings(session, surface)?;
    // Un fallo de la barra superior no debe llevarse por delante el resto.
    if let Err(error) = crate::menubar::render_menubar(session, surface) {
        eprintln!("Dokka no pudo dibujar la barra superior: {error}");
    }
    Ok(())
}

fn dock_scene(session: &mut DockSession) -> Scene {
    let images = session
        .state
        .apps()
        .iter()
        .filter_map(|app| {
            let image = session.icons.icon_for(&app.info.executable)?;
            Some((app.info.id.clone(), image))
        })
        .collect();
    let names = session
        .state
        .apps()
        .iter()
        .filter_map(|app| {
            let name = session.names.name_for(&app.info.executable)?;
            Some((app.info.id.clone(), name))
        })
        .collect();
    session.dock.scene(session.state.apps(), &images, &names)
}

/// Dibuja la cuadrícula del Launchpad en su ventana propia.
/// Solo resuelve y trata las baldosas de la página visible: el resto sigue en
/// caché entre fotogramas y entre aperturas.
fn render_launchpad(
    session: &mut DockSession,
    surface: &mut Surface,
) -> Result<(), Box<dyn Error>> {
    let size = launchpad_size(surface);
    let visible = session.launchpad.visible_apps(size);
    let raw: HashMap<AppId, Arc<Bitmap>> = visible
        .iter()
        .filter_map(|app| {
            let image = session.icons.icon_for(&app.executable)?;
            Some((app.id.clone(), image))
        })
        .collect();
    let theme = Arc::clone(&session.theme);
    let images = session.launchpad.tiled_images_for(&visible, &raw, &theme);
    let scene = launchpad_scene(&session.theme, size, &mut session.launchpad, &images);
    session.renderer.draw_launchpad(&scene)?;
    Ok(())
}
