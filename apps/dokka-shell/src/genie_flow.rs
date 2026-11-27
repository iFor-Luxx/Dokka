// Adaptado de https://github.com/crafter-station/Muelle (commit 37e843d).
// MIT License
//
// Copyright (c) 2026 Muelle contributors
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Vía inmediata del genio: al avisar Windows, la imagen viva va primero.
//!
//! Como `App::on_minimize_start` de Muelle: si la ventana es de las que sigue
//! el dock y no es de Dokka ni de Preferencias, primero se pone la imagen viva,
//! luego se sincroniza, se recoloca el dock y se apunta el vuelo al hueco
//! definitivo (o a la barrita si está oculto).

use std::error::Error;

use dokka_win_control::foreground_window;
use dokka_win_surface::Surface;

use crate::dock_session::DockSession;

/// Una ventana empezó a minimizarse: imagen viva ya, hueco después.
/// Como Muelle, la ventana sigue su curso normal si algo falla.
pub(crate) fn on_minimize_started(
    session: &mut DockSession,
    surface: &mut Surface,
    raw: u64,
) -> Result<(), Box<dyn Error>> {
    let hwnd = raw as isize;
    // No animar lo propio: Preferencias ni el dock.
    if session.is_settings_window_id(raw) {
        eprintln!("genio: sin vuelo para {hwnd}: es ventana propia");
        return Ok(());
    }
    let tracked = session
        .state
        .apps()
        .iter()
        .flat_map(|app| app.windows.iter())
        .any(|window| window.id.value() as isize == hwnd);
    if !tracked {
        eprintln!("genio: sin vuelo para {hwnd}: el dock no la sigue");
        return Ok(());
    }
    let position = session.preferences.dock.position;
    let foreground = foreground_window().map_or(0, |id| id.value() as isize);
    let Some(frame) = session.genie.begin_stand_in(position, hwnd, foreground) else {
        return Ok(());
    };
    // Ahora que la imagen tapa el hueco, sincronizar y recolocar.
    let arranging = std::time::Instant::now();
    if let Err(error) = session.sync() {
        eprintln!("genio: vuelo {hwnd} sin apuntar: no se sincronizo ({error})");
        return Ok(());
    }
    if let Err(error) = session.resize(surface) {
        eprintln!("genio: vuelo {hwnd} sin apuntar: no se recoloco ({error})");
        return Ok(());
    }
    session.genie.remember_rects(session.state.apps());
    let aspect = (frame[2] - frame[0]) as f32 / (frame[3] - frame[1]).max(1) as f32;
    let (genie, dock, state) = (&mut session.genie, &session.dock, &session.state);
    genie.aim_with_aspect(dock, state.apps(), surface, hwnd, aspect);
    eprintln!(
        "genio: tiempos {hwnd}: releer y recolocar {} ms",
        arranging.elapsed().as_millis()
    );
    Ok(())
}

/// Pasa al puente el estado ya sincronizado de la sesión.
pub(crate) fn refresh(session: &mut DockSession) {
    session.genie.on_refresh(session.state.apps());
}
