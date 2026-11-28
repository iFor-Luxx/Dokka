//! Salida del centro en bloque hacia su icono con animaciones de Composition.
//!
//! Todo el centro se encoge y se desvanece a la vez hacia su botón de la
//! barra, como una sola pieza. Cada grupo (`overlay_piece_groups.rs`) encoge
//! alrededor de su centro y lo acerca al icono en la misma proporción, así el
//! conjunto no se separa. Las anima Composition; Dokka no repinta mientras
//! duran. Se lanzan una sola vez al cerrar (`start`); si las piezas ya no
//! coinciden, falla para que el shell cierre sin animación.

use dokka_scene::{ControlDismiss, Panel};
use windows::Foundation::TimeSpan;
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::{HSTRING, Result};
use windows_numerics::Vector3;

use crate::overlay_piece_groups::{PieceGroups, piece_center, piece_middle};

/// A mitad del tiempo se ha recorrido poco y luego acelera: vuelve al botón.
const EASE_MID_PROGRESS: f32 = 0.5;
const EASE_MID_VALUE: f32 = 0.3;
/// El cristal empieza a desvanecerse casi desde el principio.
const OPACITY_HOLD: f32 = 0.1;

impl PieceGroups {
    /// Lanza la salida: todas las piezas a la vez hacia el icono.
    pub(crate) fn start(
        &self,
        compositor: &Compositor,
        tiles: &[Panel],
        dismiss: &ControlDismiss,
    ) -> Result<()> {
        if self.len() != tiles.len() {
            return Err(windows::core::Error::from_thread());
        }
        for (index, group) in self.groups().iter().enumerate() {
            let Some(tile) = tiles.get(index) else {
                return Err(windows::core::Error::from_thread());
            };
            let (middle_x, middle_y) = piece_middle(tile);
            group.SetCenterPoint(piece_center(tile))?;
            // Acercar cada centro en la misma proporción en que encoge
            // mantiene la forma del conjunto.
            let travel = 1.0 - dismiss.end_scale;
            start_piece(
                compositor,
                group,
                (dismiss.target.0 - middle_x) * travel,
                (dismiss.target.1 - middle_y) * travel,
                dismiss,
            )?;
        }
        Ok(())
    }
}

/// Anima un grupo: desvanecido, desplazamiento y escala que aceleran. Todo por fotogramas clave de Composition.
fn start_piece(
    compositor: &Compositor,
    group: &ContainerVisual,
    dx: f32,
    dy: f32,
    dismiss: &ControlDismiss,
) -> Result<()> {
    let opacity = compositor.CreateScalarKeyFrameAnimation()?;
    opacity.InsertKeyFrame(0.0, 1.0)?;
    opacity.InsertKeyFrame(OPACITY_HOLD, 1.0)?;
    opacity.InsertKeyFrame(1.0, 0.0)?;
    opacity.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Opacity"), &opacity)?;

    let shift = compositor.CreateVector3KeyFrameAnimation()?;
    shift.InsertKeyFrame(0.0, Vector3::default())?;
    shift.InsertKeyFrame(
        EASE_MID_PROGRESS,
        Vector3 {
            X: dx * EASE_MID_VALUE,
            Y: dy * EASE_MID_VALUE,
            Z: 0.0,
        },
    )?;
    shift.InsertKeyFrame(
        1.0,
        Vector3 {
            X: dx,
            Y: dy,
            Z: 0.0,
        },
    )?;
    shift.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Offset"), &shift)?;

    let end = dismiss.end_scale;
    let middle = 1.0 + (end - 1.0) * EASE_MID_VALUE;
    let scale = compositor.CreateVector3KeyFrameAnimation()?;
    scale.InsertKeyFrame(
        0.0,
        Vector3 {
            X: 1.0,
            Y: 1.0,
            Z: 1.0,
        },
    )?;
    scale.InsertKeyFrame(
        EASE_MID_PROGRESS,
        Vector3 {
            X: middle,
            Y: middle,
            Z: 1.0,
        },
    )?;
    scale.InsertKeyFrame(
        1.0,
        Vector3 {
            X: end,
            Y: end,
            Z: 1.0,
        },
    )?;
    scale.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Scale"), &scale)
}

/// Segundos a la unidad de Composition (intervalos de 100 ns).
fn span(seconds: f32) -> TimeSpan {
    TimeSpan {
        Duration: (seconds * 10_000_000.0) as i64,
    }
}
