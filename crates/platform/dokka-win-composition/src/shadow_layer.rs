//! Sombra genuina del compositor para un panel de cristal: `SpriteVisual` sin
//! pincel con `DropShadow` enmascarado por la forma vectorial, debajo del
//! cristal y con clip en anillo (geometría D2D combinada). Falla sola.

use dokka_scene::{Panel, Shadow};
use windows::UI::Color;
use windows::UI::Composition::{
    CompositionGeometricClip, CompositionRoundedRectangleGeometry, Compositor, ContainerVisual,
    DropShadow, ShapeVisual, SpriteVisual,
};
use windows::Win32::Graphics::Direct2D::{
    D2D1_FACTORY_TYPE_SINGLE_THREADED, D2D1CreateFactory, ID2D1Factory,
};
use windows::core::Result;
use windows_numerics::{Vector2, Vector3};

use crate::shadow_geometry::build_path;
use crate::windows_color::windows_color;

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
        let factory: ID2D1Factory =
            unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)? };
        let margin = shadow_margin(shadow);
        let path_geometry = build_path(compositor, &factory, panel, occluders, margin)?;
        let mask_geometry = compositor.CreateRoundedRectangleGeometry()?;
        mask_geometry.SetCornerRadius(Vector2 {
            X: panel.corner_radius,
            Y: panel.corner_radius,
        })?;
        mask_geometry.SetSize(Vector2 {
            X: panel.bounds.width,
            Y: panel.bounds.height,
        })?;
        let mask_shape = compositor.CreateSpriteShapeWithGeometry(&mask_geometry)?;
        let white = compositor.CreateColorBrushWithColor(Color {
            A: 255,
            R: 255,
            G: 255,
            B: 255,
        })?;
        mask_shape.SetFillBrush(&white)?;
        let mask_visual = compositor.CreateShapeVisual()?;
        mask_visual.Shapes()?.Append(&mask_shape)?;
        mask_visual.SetSize(Vector2 {
            X: panel.bounds.width,
            Y: panel.bounds.height,
        })?;
        let mask_surface = compositor.CreateVisualSurface()?;
        mask_surface.SetSourceVisual(&mask_visual)?;
        mask_surface.SetSourceSize(Vector2 {
            X: panel.bounds.width,
            Y: panel.bounds.height,
        })?;
        let mask_brush = compositor.CreateSurfaceBrushWithSurface(&mask_surface)?;
        let drop = compositor.CreateDropShadow()?;
        drop.SetColor(windows_color(shadow.color))?;
        drop.SetBlurRadius(shadow.blur)?;
        drop.SetOffset(Vector3 {
            X: shadow.offset.0,
            Y: shadow.offset.1,
            Z: 0.0,
        })?;
        drop.SetOpacity(1.0)?;
        drop.SetMask(&mask_brush)?;
        let clip = compositor.CreateGeometricClipWithGeometry(&path_geometry)?;
        let visual = compositor.CreateSpriteVisual()?;
        visual.SetOffset(Vector3 {
            X: panel.bounds.x,
            Y: panel.bounds.y,
            Z: 0.0,
        })?;
        visual.SetSize(Vector2 {
            X: panel.bounds.width,
            Y: panel.bounds.height,
        })?;
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
        self.visual.SetOffset(Vector3 {
            X: panel.bounds.x,
            Y: panel.bounds.y,
            Z: 0.0,
        })?;
        let size = Vector2 {
            X: panel.bounds.width,
            Y: panel.bounds.height,
        };
        self.visual.SetSize(size)?;
        self.mask_visual.SetSize(size)?;
        self.mask_geometry.SetSize(size)?;
        self.mask_geometry.SetCornerRadius(Vector2 {
            X: panel.corner_radius,
            Y: panel.corner_radius,
        })?;
        self.mask_surface.SetSourceSize(size)?;
        if self.shadow.color != shadow.color {
            self.drop.SetColor(windows_color(shadow.color))?;
        }
        if self.shadow.blur != shadow.blur {
            self.drop.SetBlurRadius(shadow.blur)?;
        }
        if self.shadow.offset != shadow.offset {
            self.drop.SetOffset(Vector3 {
                X: shadow.offset.0,
                Y: shadow.offset.1,
                Z: 0.0,
            })?;
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
    tiles
        .iter()
        .enumerate()
        .filter(|(other, _)| *other != index)
        .map(|(_, other)| *other)
        .collect()
}

/// Aire alrededor del panel donde cabe el difuminado más el desplazamiento.
fn shadow_margin(shadow: &Shadow) -> f32 {
    (shadow.blur + shadow.offset.0.abs().max(shadow.offset.1.abs()) + 4.0).ceil()
}

/// `true` si el clip no necesita rehacerse por este panel.
fn same_shape(old: &Panel, new: &Panel) -> bool {
    old.bounds == new.bounds && old.corner_radius == new.corner_radius
}

/// `true` si los paneles que se excluyen no cambiaron de forma.
fn same_occluders(old: &[Panel], new: &[Panel]) -> bool {
    old.len() == new.len() && old.iter().zip(new).all(|(a, b)| same_shape(a, b))
}
