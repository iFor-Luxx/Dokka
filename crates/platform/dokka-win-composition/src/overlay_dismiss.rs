//! Piezas del panel flotante como objetos propios para su salida.
//!
//! Cada pieza (`tiles`) lleva lo que cae dentro (planos, textos, iconos,
//! líneas, puntos y cursor) en su mismo contenedor: al cerrarse el centro,
//! el grupo vuela entero hacia el icono, encogiéndose y desvaneciéndose
//! alrededor del centro de la pieza. Los grupos se crean al dibujar (sin
//! cambiar lo visible: empiezan en identidad) y solo se animan al cerrar,
//! una vez, con animaciones de Composition por fotogramas clave.

use dokka_scene::{ControlDismiss, Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::{HSTRING, Result};
use windows::Foundation::TimeSpan;
use windows_numerics::Vector3;

/// Suavizado a mitad de camino (salida cubicada al 55%): llegada blanda.
const EASE_MID_PROGRESS: f32 = 0.55;
/// Fracción recorrida al `EASE_MID_PROGRESS` con salida cubicada.
const EASE_MID_VALUE: f32 = 0.909;
/// La opacidad se mantiene antes de desvanecerse, como el fundido anterior.
const OPACITY_HOLD: f32 = 0.35;

/// A qué pieza pertenece cada capa que cae dentro de una (`None`: fondo).
pub(crate) struct GroupMap {
    flats: Vec<Option<usize>>,
    dividers: Vec<Option<usize>>,
    cursors: Vec<Option<usize>>,
    dots: Vec<Option<usize>>,
    texts: Vec<Option<usize>>,
    icons: Vec<Option<usize>>,
}

/// Un contenedor por pieza, en el orden de `tiles`, en identidad.
pub(crate) struct PieceGroups {
    groups: Vec<ContainerVisual>,
}

impl PieceGroups {
    /// Crea un contenedor por pieza y dice a cuál pertenece cada capa
    /// interior por contención geométrica. Sin piezas no crea ninguno.
    pub(crate) fn build(
        compositor: &Compositor,
        container: &ContainerVisual,
        overlay: &Overlay,
    ) -> Result<(Self, GroupMap)> {
        let mut groups = Vec::with_capacity(overlay.tiles.len());
        for tile in &overlay.tiles {
            let group = compositor.CreateContainerVisual()?;
            group.SetCenterPoint(piece_center(tile))?;
            container.Children()?.InsertAtTop(&group)?;
            groups.push(group);
        }
        let at = |point| tile_of(&overlay.tiles, point);
        let map = GroupMap {
            flats: overlay.flat_panels.iter().map(|panel| at(rect_center(&panel.bounds))).collect(),
            dividers: overlay.dividers.iter().map(|divider| at(rect_center(&divider.bounds))).collect(),
            cursors: overlay.cursors.iter().map(|cursor| at((cursor.text_x, cursor.center_y))).collect(),
            dots: overlay.dots.iter().map(|dot| {
                at((dot.x + dot.diameter / 2.0, dot.y + dot.diameter / 2.0))
            }).collect(),
            texts: overlay.texts.iter().map(|text| at((text.x, text.center_y))).collect(),
            icons: overlay.icons.iter().map(|icon| {
                at((icon.x + icon.side / 2.0, icon.y + icon.side / 2.0))
            }).collect(),
        };
        Ok((Self { groups }, map))
    }

    /// Contenedor de esta pieza, o el fondo si no existe.
    pub(crate) fn parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        index: usize,
    ) -> &'a ContainerVisual {
        self.groups.get(index).map_or(container, |group| group)
    }

    /// Contenedor de la pieza indicada, o el fondo sin pieza.
    pub(crate) fn parent_opt<'a>(
        &'a self,
        container: &'a ContainerVisual,
        index: Option<usize>,
    ) -> &'a ContainerVisual {
        index.map_or(container, |known| self.parent(container, known))
    }

    /// Padre para cada plano, línea, cursor, punto, texto o icono, en orden.
    pub(crate) fn flat_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.flats.get(index).copied().flatten())
    }

    pub(crate) fn divider_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.dividers.get(index).copied().flatten())
    }

    pub(crate) fn cursor_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.cursors.get(index).copied().flatten())
    }

    pub(crate) fn dot_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.dots.get(index).copied().flatten())
    }

    pub(crate) fn text_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.texts.get(index).copied().flatten())
    }

    pub(crate) fn icon_parent<'a>(
        &'a self,
        container: &'a ContainerVisual,
        map: &GroupMap,
        index: usize,
    ) -> &'a ContainerVisual {
        self.parent_opt(container, map.icons.get(index).copied().flatten())
    }

    /// Lanza la salida por piezas: cada grupo vuela a su icono,
    /// encogiéndose y desvaneciéndose, primero el más cercano.
    /// Las anima Composition; Dokka no repinta mientras duran.
    pub(crate) fn start(
        &self,
        compositor: &Compositor,
        tiles: &[Panel],
        dismiss: &ControlDismiss,
    ) -> Result<()> {
        if self.groups.len() != tiles.len() {
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
        for (index, group) in self.groups.iter().enumerate() {
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
    shift.InsertKeyFrame(EASE_MID_PROGRESS, Vector3 { X: dx * EASE_MID_VALUE, Y: dy * EASE_MID_VALUE, Z: 0.0 })?;
    shift.InsertKeyFrame(1.0, Vector3 { X: dx, Y: dy, Z: 0.0 })?;
    shift.SetDelayTime(span(delay))?;
    shift.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Offset"), &shift)?;

    let end = dismiss.end_scale;
    let middle = 1.0 + (end - 1.0) * EASE_MID_VALUE;
    let scale = compositor.CreateVector3KeyFrameAnimation()?;
    scale.InsertKeyFrame(0.0, Vector3 { X: 1.0, Y: 1.0, Z: 1.0 })?;
    scale.InsertKeyFrame(EASE_MID_PROGRESS, Vector3 { X: middle, Y: middle, Z: 1.0 })?;
    scale.InsertKeyFrame(1.0, Vector3 { X: end, Y: end, Z: 1.0 })?;
    scale.SetDelayTime(span(delay))?;
    scale.SetDuration(span(dismiss.duration))?;
    group.StartAnimation(&HSTRING::from("Scale"), &scale)
}

/// Centro de la pieza como punto de escala, en unidades lógicas.
fn piece_center(tile: &Panel) -> Vector3 {
    let (x, y) = piece_middle(tile);
    Vector3 { X: x, Y: y, Z: 0.0 }
}

/// Centro geométrico de la pieza, en unidades lógicas.
fn piece_middle(tile: &Panel) -> (f32, f32) {
    (
        tile.bounds.x + tile.bounds.width / 2.0,
        tile.bounds.y + tile.bounds.height / 2.0,
    )
}

/// Centro de un rectángulo, en unidades lógicas.
fn rect_center(bounds: &dokka_scene::Rect) -> (f32, f32) {
    (
        bounds.x + bounds.width / 2.0,
        bounds.y + bounds.height / 2.0,
    )
}

/// Primera pieza que contiene el punto, si alguna (no se solapan).
fn tile_of(tiles: &[Panel], point: (f32, f32)) -> Option<usize> {
    tiles.iter().position(|tile| {
        let bounds = tile.bounds;
        point.0 >= bounds.x
            && point.0 <= bounds.x + bounds.width
            && point.1 >= bounds.y
            && point.1 <= bounds.y + bounds.height
    })
}

/// Segundos a la unidad de Composition (intervalos de 100 ns).
fn span(seconds: f32) -> TimeSpan {
    TimeSpan {
        Duration: (seconds * 10_000_000.0) as i64,
    }
}
