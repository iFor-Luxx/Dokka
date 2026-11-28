//! Salida del centro por piezas con animaciones de Composition.
//!
//! Cada grupo (`overlay_piece_groups.rs`) vuela a su icono, encogiéndose y
//! desvaneciéndose alrededor del centro de la pieza, primero el más cercano.
//! Las anima Composition; Dokka no repinta mientras duran. Se lanzan una
//! sola vez al cerrar (`start`); si las piezas ya no coinciden, falla para
//! que el shell cierre sin animación.

use dokka_scene::{ControlDismiss, Panel};
use windows::Foundation::TimeSpan;
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::{HSTRING, Result};
use windows_numerics::Vector3;

use crate::overlay_piece_groups::{PieceGroups, piece_center, piece_middle};

/// Suavizado a mitad de camino (salida cubicada al 55%): llegada blanda.
const EASE_MID_PROGRESS: f32 = 0.55;
/// Fracción recorrida al `EASE_MID_PROGRESS` con salida cubicada.
const EASE_MID_VALUE: f32 = 0.909;
/// La opacidad se mantiene antes de desvanecerse, como el fundido anterior.
const OPACITY_HOLD: f32 = 0.35;

impl PieceGroups {
    /// Lanza la salida por piezas: cada grupo vuela a su icono,
    /// encogiéndose y desvaneciéndose, primero el más cercano.
    /// Las anima Composition; Dokka no repinta mientras duran.
    pub(crate) fn start(
        &self,
        compositor: &Compositor,
        tiles: &[Panel],
        dismiss: &ControlDismiss,
    ) -> Result<()> {
        if self.len() != tiles.len() {
            return Err(windows::core::Error::from_thread());
        }
        let mut by_distance: Vec<(f32, usize)> = tiles
            .iter()
            .enumerate()
            .map(|(index, tile)| {
                let (x, y) = piece_middle(tile);
                let dx = dismiss.target.0 - x;
                let dy = dismiss.target.1 - y;
                (dx * dx + dy * dy, index)
            })
            .collect();
        by_distance.sort_by(|first, second| first.0.total_cmp(&second.0));
        let mut delay_of = vec![0.0f32; tiles.len()];
        for (rank, &(_, index)) in by_distance.iter().enumerate() {
            delay_of[index] = dismiss.stagger * rank as f32;
        }
        for (index, group) in self.groups().iter().enumerate() {
            let Some(tile) = tiles.get(index) else {
                return Err(windows::core::Error::from_thread());
            };
            let (middle_x, middle_y) = piece_middle(tile);
            group.SetCenterPoint(piece_center(tile))?;
            let delay = delay_of.get(index).copied().unwrap_or(0.0);
            start_piece(
                compositor,
                group,
                dismiss.target.0 - middle_x,
                dismiss.target.1 - middle_y,
                delay,
                dismiss,
            )?;
        }
        Ok(())
    }
}

/// Anima un grupo: opacidad con espera y desvanecido, desplazamiento y
/// escala con llegada blanda. Todo por fotogramas clave de Composition.
fn start_piece(
    compositor: &Compositor,
    group: &ContainerVisual,
    dx: f32,
    dy: f32,
    delay: f32,
    dismiss: &ControlDismiss,
) -> Result<()> {
    let opacity = compositor.CreateScalarKeyFrameAnimation()?;
    opacity.InsertKeyFrame(0.0, 1.0)?;
    opacity.InsertKeyFrame(OPACITY_HOLD, 1.0)?;
    opacity.InsertKeyFrame(1.0, 0.0)?;
    opacity.SetDelayTime(span(delay))?;
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
    shift.SetDelayTime(span(delay))?;
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
    scale.SetDelayTime(span(delay))?;
    scale.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Scale"), &scale)
}

/// Segundos a la unidad de Composition (intervalos de 100 ns).
fn span(seconds: f32) -> TimeSpan {
    TimeSpan {
        Duration: (seconds * 10_000_000.0) as i64,
    }
}
