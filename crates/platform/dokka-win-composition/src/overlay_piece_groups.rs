//! Grupos de piezas del panel flotante para su salida por piezas.
//!
//! Cada pieza (`tiles`) lleva lo que cae dentro (planos, textos, iconos,
//! líneas, puntos y cursor) en su mismo contenedor: al cerrarse el centro,
//! el grupo vuela entero hacia el icono, encogiéndose y desvaneciéndose
//! alrededor del centro de la pieza. Los grupos se crean al dibujar (sin
//! cambiar lo visible: empiezan en identidad); la animación vive en
//! `overlay_dismiss.rs`.

use dokka_scene::{Overlay, Panel};
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::core::Result;
use windows_numerics::Vector3;

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
    /// Número de grupos, en el orden de `tiles`.
    pub(crate) fn len(&self) -> usize {
        self.groups.len()
    }

    /// Grupos en el orden de `tiles`, para animarlos al cerrar.
    pub(crate) fn groups(&self) -> &[ContainerVisual] {
        &self.groups
    }

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
            flats: overlay
                .flat_panels
                .iter()
                .map(|panel| at(rect_center(&panel.bounds)))
                .collect(),
            dividers: overlay
                .dividers
                .iter()
                .map(|divider| at(rect_center(&divider.bounds)))
                .collect(),
            cursors: overlay
                .cursors
                .iter()
                .map(|cursor| at((cursor.text_x, cursor.center_y)))
                .collect(),
            dots: overlay
                .dots
                .iter()
                .map(|dot| at((dot.x + dot.diameter / 2.0, dot.y + dot.diameter / 2.0)))
                .collect(),
            texts: overlay
                .texts
                .iter()
                .map(|text| at((text.x, text.center_y)))
                .collect(),
            icons: overlay
                .icons
                .iter()
                .map(|icon| at((icon.x + icon.side / 2.0, icon.y + icon.side / 2.0)))
                .collect(),
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
}

/// Centro de la pieza como punto de escala, en unidades lógicas.
pub(crate) fn piece_center(tile: &Panel) -> Vector3 {
    let (x, y) = piece_middle(tile);
    Vector3 { X: x, Y: y, Z: 0.0 }
}

/// Centro geométrico de la pieza, en unidades lógicas.
pub(crate) fn piece_middle(tile: &Panel) -> (f32, f32) {
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
