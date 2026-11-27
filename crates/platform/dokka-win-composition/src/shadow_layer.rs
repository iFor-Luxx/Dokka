//! Sombra genuina del compositor para un panel de cristal: `SpriteVisual` sin
//! pincel con `DropShadow` enmascarado por la forma vectorial, debajo del
//! cristal y con clip en anillo (geometría D2D combinada). Falla sola.

use dokka_scene::{Panel, Shadow};
use windows::Graphics::{IGeometrySource2D, IGeometrySource2D_Impl};
use windows::UI::Color;
use windows::UI::Composition::{
    CompositionGeometricClip, CompositionPath, CompositionPathGeometry,
    CompositionRoundedRectangleGeometry, Compositor, ContainerVisual, DropShadow, ShapeVisual,
    SpriteVisual,
};
use windows::Win32::Graphics::Direct2D::Common::{D2D_RECT_F, D2D1_FILL_MODE_ALTERNATE};
use windows::Win32::Graphics::Direct2D::{
    D2D1_COMBINE_MODE_EXCLUDE, D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1_ROUNDED_RECT,
    D2D1CreateFactory, ID2D1Factory, ID2D1Geometry,
};
use windows::Win32::System::WinRT::Graphics::Direct2D::{
    IGeometrySource2DInterop, IGeometrySource2DInterop_Impl,
};
use windows::core::{Interface, Ref, Result, implement};
use windows_numerics::{Vector2, Vector3};

use crate::windows_color::windows_color;

/// Fuente de geometría 2D que envuelve la D2D combinada para `CompositionPath`.
#[implement(IGeometrySource2D, IGeometrySource2DInterop)]
struct GeometrySource {
    geometry: ID2D1Geometry,
}

impl IGeometrySource2D_Impl for GeometrySource {}

impl IGeometrySource2DInterop_Impl for GeometrySource {
    fn GetGeometry(&self) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }

    fn TryGetGeometryUsingFactory(&self, _factory: Ref<ID2D1Factory>) -> Result<ID2D1Geometry> {
        Ok(self.geometry.clone())
    }
}

/// Sombra bajo un cristal, con su misma forma redondeada y sin tapar dentro.
pub(crate) struct ShadowLayer {
    visual: SpriteVisual,
    drop: DropShadow,
    mask_visual: ShapeVisual,
    mask_geometry: CompositionRoundedRectangleGeometry,
    mask_surface: windows::UI::Composition::CompositionVisualSurface,
    clip: CompositionGeometricClip,
    factory: ID2D1Factory,
    panel: Panel,
    shadow: Shadow,
    occluders: Vec<Panel>,
    margin: f32,
}

impl ShadowLayer {
    /// Crea la sombra bajo el cristal, insertada debajo para no taparlo.
    /// `occluders` son los paneles vecinos que la sombra no debe oscurecer.
    pub(crate) fn add(
        compositor: &Compositor,
        parent: &ContainerVisual,
        panel: &Panel,
        shadow: &Shadow,
        occluders: &[Panel],
    ) -> Result<Self> {
        if panel.bounds.width <= 0.0 || panel.bounds.height <= 0.0 {
            return Err(windows::core::Error::from_thread());
        }
        // SAFETY: la factoría solo se usa en este hilo para geometría 2D.
        let factory: ID2D1Factory = unsafe {
            D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?
        };
        let margin = shadow_margin(shadow);
        let path_geometry = build_path(compositor, &factory, panel, occluders, margin)?;
        let mask_geometry = compositor.CreateRoundedRectangleGeometry()?;
        mask_geometry.SetCornerRadius(Vector2 { X: panel.corner_radius, Y: panel.corner_radius })?;
        mask_geometry.SetSize(Vector2 { X: panel.bounds.width, Y: panel.bounds.height })?;
        let mask_shape = compositor.CreateSpriteShapeWithGeometry(&mask_geometry)?;
        let white = compositor.CreateColorBrushWithColor(Color { A: 255, R: 255, G: 255, B: 255 })?;
        mask_shape.SetFillBrush(&white)?;
        let mask_visual = compositor.CreateShapeVisual()?;
        mask_visual.Shapes()?.Append(&mask_shape)?;
        mask_visual.SetSize(Vector2 { X: panel.bounds.width, Y: panel.bounds.height })?;
        let mask_surface = compositor.CreateVisualSurface()?;
        mask_surface.SetSourceVisual(&mask_visual)?;
        mask_surface.SetSourceSize(Vector2 { X: panel.bounds.width, Y: panel.bounds.height })?;
        let mask_brush = compositor.CreateSurfaceBrushWithSurface(&mask_surface)?;
        let drop = compositor.CreateDropShadow()?;
        drop.SetColor(windows_color(shadow.color))?;
        drop.SetBlurRadius(shadow.blur)?;
        drop.SetOffset(Vector3 { X: shadow.offset.0, Y: shadow.offset.1, Z: 0.0 })?;
        drop.SetOpacity(1.0)?;
        drop.SetMask(&mask_brush)?;
        let clip = compositor.CreateGeometricClipWithGeometry(&path_geometry)?;
        let visual = compositor.CreateSpriteVisual()?;
        visual.SetOffset(Vector3 { X: panel.bounds.x, Y: panel.bounds.y, Z: 0.0 })?;
        visual.SetSize(Vector2 { X: panel.bounds.width, Y: panel.bounds.height })?;
        visual.SetClip(&clip)?;
        visual.SetShadow(&drop)?;
        parent.Children()?.InsertAtBottom(&visual)?;
        Ok(Self {
            visual,
            drop,
            mask_visual,
            mask_geometry,
            mask_surface,
            clip,
            factory,
            panel: *panel,
            shadow: *shadow,
            occluders: occluders.to_vec(),
            margin,
        })
    }

    /// Mueve y redimensiona sin reconstruir; solo rehace el clip si cambia la
    /// forma y solo toca el color, el desenfoque o el desplazamiento si cambian.
    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        panel: &Panel,
        shadow: &Shadow,
        occluders: &[Panel],
    ) -> Result<()> {
        self.visual.SetOffset(Vector3 { X: panel.bounds.x, Y: panel.bounds.y, Z: 0.0 })?;
        let size = Vector2 { X: panel.bounds.width, Y: panel.bounds.height };
        self.visual.SetSize(size)?;
        self.mask_visual.SetSize(size)?;
        self.mask_geometry.SetSize(size)?;
        self.mask_geometry.SetCornerRadius(Vector2 { X: panel.corner_radius, Y: panel.corner_radius })?;
        self.mask_surface.SetSourceSize(size)?;
        if self.shadow.color != shadow.color {
            self.drop.SetColor(windows_color(shadow.color))?;
        }
        if self.shadow.blur != shadow.blur {
            self.drop.SetBlurRadius(shadow.blur)?;
        }
        if self.shadow.offset != shadow.offset {
            self.drop.SetOffset(Vector3 { X: shadow.offset.0, Y: shadow.offset.1, Z: 0.0 })?;
        }
        let margin = shadow_margin(shadow);
        if same_shape(&self.panel, panel)
            && same_occluders(&self.occluders, occluders)
            && self.margin == margin
        {
            self.shadow = *shadow;
            return Ok(());
        }
        let path_geometry = build_path(compositor, &self.factory, panel, occluders, margin)?;
        self.clip.SetGeometry(&path_geometry)?;
        self.panel = *panel;
        self.shadow = *shadow;
        self.occluders = occluders.to_vec();
        self.margin = margin;
        Ok(())
    }
}

/// Todas las piezas menos la `index`: lo que su sombra debe excluir.
pub(crate) fn others_of(tiles: &[Panel], index: usize) -> Vec<Panel> {
    tiles.iter().enumerate().filter(|(other, _)| *other != index).map(|(_, other)| *other).collect()
}

/// Aire alrededor del panel donde cabe el difuminado más el desplazamiento.
fn shadow_margin(shadow: &Shadow) -> f32 {    (shadow.blur + shadow.offset.0.abs().max(shadow.offset.1.abs()) + 4.0).ceil()
}

/// `true` si el clip no necesita rehacerse por este panel.
fn same_shape(old: &Panel, new: &Panel) -> bool {
    old.bounds == new.bounds && old.corner_radius == new.corner_radius
}

/// `true` si los paneles que se excluyen no cambiaron de forma.
fn same_occluders(old: &[Panel], new: &[Panel]) -> bool {
    old.len() == new.len()
        && old
            .iter()
            .zip(new)
            .all(|(a, b)| same_shape(a, b))
}

/// Anillo exterior menos interior (y vecinos): rectángulo fuera, redondeados
/// dentro, en coordenadas del propio visual. Todo vectorial, sin imágenes.
fn build_path(
    compositor: &Compositor,
    factory: &ID2D1Factory,
    panel: &Panel,
    occluders: &[Panel],
    margin: f32,
) -> Result<CompositionPathGeometry> {
    let width = panel.bounds.width;
    let height = panel.bounds.height;
    let radius = panel
        .corner_radius
        .min(width / 2.0)
        .min(height / 2.0)
        .max(0.0);
    // SAFETY: la factoría y las geometrías solo se usan en este hilo; el
    // `sink` se cierra siempre antes de crear el camino de composición.
    let d2d = unsafe {
        let outer = D2D_RECT_F { left: -margin, top: -margin, right: width + margin, bottom: height + margin };
        let outer_geo = factory.CreateRectangleGeometry(&outer)?;
        let mut inners: Vec<Option<ID2D1Geometry>> = Vec::new();
        let inner = D2D1_ROUNDED_RECT {
            rect: D2D_RECT_F { left: 0.0, top: 0.0, right: width, bottom: height },
            radiusX: radius,
            radiusY: radius,
        };
        let inner_geo: ID2D1Geometry = factory
            .CreateRoundedRectangleGeometry(&inner)?
            .cast()?;
        inners.push(Some(inner_geo));
        for occ in occluders {
            if occ.bounds.width <= 0.0 || occ.bounds.height <= 0.0 {
                continue;
            }
            let local_x = occ.bounds.x - panel.bounds.x;
            let local_y = occ.bounds.y - panel.bounds.y;
            let oradius = occ
                .corner_radius
                .min(occ.bounds.width / 2.0)
                .min(occ.bounds.height / 2.0)
                .max(0.0);
            let rounded = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F { left: local_x, top: local_y, right: local_x + occ.bounds.width, bottom: local_y + occ.bounds.height },
                radiusX: oradius,
                radiusY: oradius,
            };
            let geo: ID2D1Geometry = factory
                .CreateRoundedRectangleGeometry(&rounded)?
                .cast()?;
            inners.push(Some(geo));
        }
        let group = factory.CreateGeometryGroup(D2D1_FILL_MODE_ALTERNATE, &inners)?;
        let path_geo = factory.CreatePathGeometry()?;
        let sink = path_geo.Open()?;
        outer_geo.CombineWithGeometry(&group, D2D1_COMBINE_MODE_EXCLUDE, None, 0.25, &sink)?;
        sink.Close()?;
        let combined: ID2D1Geometry = path_geo.cast()?;
        combined
    };
    let source: IGeometrySource2D = GeometrySource { geometry: d2d }.into();
    let path = CompositionPath::Create(&source)?;
    compositor.CreatePathGeometryWithPath(&path)
}
