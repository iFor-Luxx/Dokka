//! Franja difuminada bajo la cabecera de Preferencias, estilo Tahoe.
//!
//! Entre el contenido y la cabecera: desenfoque real de lo que pasa por
//! debajo (pincel de fondo de la propia ventana) más tinte en degradado.
//! La píldora y el título van por encima, nítidos. Se crea una vez y solo
//! se actualiza si cambia; sin desenfoque queda el tinte. Solo Preferencias.

use dokka_scene::{HeaderVeil, Panel};
use windows::UI::Composition::{
    CompositionRoundedRectangleGeometry, Compositor, ContainerVisual, ShapeVisual, SpriteVisual,
};
use windows::core::{HSTRING, Result};
use windows_numerics::{Vector2, Vector3};

use crate::blur_effect::{BLUR_ANIMATABLE, SOURCE_NAME, create_blur_factory};
use crate::divider_layer::DividerLayer;
use crate::header_veil_paint::{veil_gradient, window_clip_of};
use crate::icon_layer::IconLayer;
use crate::painter::Painter;
use crate::panel_layer::PanelLayer;
use crate::scene_layers::{PanelStyle, panel_style_of};
use crate::text_layer::TextLayer;

/// Por debajo de esta intensidad la franja se oculta y se ve como hoy.
const VEIL_ACTIVE_MIN: f32 = 0.01;

/// Franja entre el contenido y la cabecera, con su primer plano por encima.
pub(crate) struct HeaderVeilLayer {
    blur_sprite: Option<SpriteVisual>,
    /// `true` si el fallo ya se escribió en consola: no se repite.
    warned: bool,
    tint: ShapeVisual,
    tint_geometry: CompositionRoundedRectangleGeometry,
    veil_clip_geometry: CompositionRoundedRectangleGeometry,
    content_clip_geometry: CompositionRoundedRectangleGeometry,
    panels: Vec<PanelLayer>,
    panel_styles: Vec<PanelStyle>,
    flats: Vec<PanelLayer>,
    flat_styles: Vec<PanelStyle>,
    dividers: Vec<DividerLayer>,
    icons: Vec<IconLayer>,
    texts: Vec<TextLayer>,
    /// Lo que obliga a reconstruir si cambia: tinte y radio.
    style: (dokka_scene::Color, u32),
}

impl HeaderVeilLayer {
    /// Crea la franja sobre el contenido y su cabecera por encima. El
    /// contenido queda recortado a la ventana redondeada para no pintar en
    /// el aire de la sombra ni fuera de ella.
    pub(crate) fn add(
        compositor: &Compositor,
        painter: &Painter,
        holder: &ContainerVisual,
        content: &ContainerVisual,
        window: &Panel,
        veil: &HeaderVeil,
    ) -> Result<Self> {
        let children = holder.Children()?;
        let veil_visual = compositor.CreateContainerVisual()?;
        children.InsertAtTop(&veil_visual)?;
        let header = compositor.CreateContainerVisual()?;
        children.InsertAtTop(&header)?;
        let veil_clip_geometry = compositor.CreateRoundedRectangleGeometry()?;
        veil_visual.SetClip(&compositor.CreateGeometricClipWithGeometry(&veil_clip_geometry)?)?;
        let content_clip_geometry = compositor.CreateRoundedRectangleGeometry()?;
        content.SetClip(&compositor.CreateGeometricClipWithGeometry(&content_clip_geometry)?)?;
        let mut layer = Self {
            blur_sprite: None,
            warned: false,
            tint: compositor.CreateShapeVisual()?,
            tint_geometry: compositor.CreateRoundedRectangleGeometry()?,
            veil_clip_geometry,
            content_clip_geometry,
            panels: Vec::with_capacity(veil.panels.len()),
            panel_styles: veil.panels.iter().map(panel_style_of).collect(),
            flats: Vec::with_capacity(veil.flats.len()),
            flat_styles: veil.flats.iter().map(panel_style_of).collect(),
            dividers: Vec::with_capacity(veil.dividers.len()),
            icons: Vec::with_capacity(veil.icons.len()),
            texts: Vec::with_capacity(veil.texts.len()),
            style: veil_style_of(veil),
        };
        for panel in &veil.panels {
            layer
                .panels
                .push(PanelLayer::add(compositor, &header, panel)?);
        }
        for panel in &veil.flats {
            layer
                .flats
                .push(PanelLayer::add_flat(compositor, &header, panel)?);
        }
        for divider in &veil.dividers {
            layer
                .dividers
                .push(DividerLayer::add(compositor, &header, divider)?);
        }
        for icon in &veil.icons {
            layer
                .icons
                .push(IconLayer::add(compositor, painter, &header, icon)?);
        }
        for _ in &veil.texts {
            layer.texts.push(TextLayer::add(compositor, &header)?);
        }
        // El desenfoque va debajo del tinte: se crea antes para quedar abajo.
        // Si no se puede crear, queda solo el tinte en degradado.
        if let Err(err) = layer.ensure_blur(compositor, &veil_visual, veil) {
            layer.warn_once(&format!("al crear el efecto: {err}"));
        }
        layer.attach_tint(compositor, &veil_visual)?;
        layer.update(compositor, painter, window, veil)?;
        Ok(layer)
    }

    /// `true` si basta actualizar: mismo tinte, radio y piezas.
    pub(crate) fn matches(&self, veil: &HeaderVeil) -> bool {
        self.style == veil_style_of(veil)
            && self.panels.len() == veil.panels.len()
            && self
                .panel_styles
                .iter()
                .zip(&veil.panels)
                .all(|(style, panel)| *style == panel_style_of(panel))
            && self.flats.len() == veil.flats.len()
            && self
                .flat_styles
                .iter()
                .zip(&veil.flats)
                .all(|(style, panel)| *style == panel_style_of(panel))
            && self.dividers.len() == veil.dividers.len()
            && self.icons.len() == veil.icons.len()
            && self
                .icons
                .iter()
                .zip(&veil.icons)
                .all(|(layer, icon)| layer.accepts(icon))
            && self.texts.len() == veil.texts.len()
    }

    /// Mueve y redimensiona la franja, ajusta su intensidad y coloca la
    /// cabecera; el degradado sigue al tamaño sin reconstruir.
    pub(crate) fn update(
        &mut self,
        compositor: &Compositor,
        painter: &Painter,
        window: &Panel,
        veil: &HeaderVeil,
    ) -> Result<()> {
        let bounds = veil.bounds;
        let width = bounds.width.max(0.0);
        let height = bounds.height.max(0.0);
        let visible = veil.strength > VEIL_ACTIVE_MIN && width > 0.0 && height > 0.0;
        self.tint.SetOffset(Vector3 {
            X: bounds.x,
            Y: bounds.y,
            Z: 0.0,
        })?;
        self.tint.SetSize(Vector2 {
            X: width,
            Y: height,
        })?;
        self.tint_geometry.SetSize(Vector2 {
            X: width,
            Y: height,
        })?;
        self.tint.SetOpacity(veil.strength)?;
        self.tint.SetIsVisible(visible)?;
        window_clip_of(&self.veil_clip_geometry, window)?;
        window_clip_of(&self.content_clip_geometry, window)?;
        if let Some(sprite) = &self.blur_sprite {
            sprite.SetOffset(Vector3 {
                X: bounds.x,
                Y: bounds.y,
                Z: 0.0,
            })?;
            sprite.SetSize(Vector2 {
                X: width,
                Y: height,
            })?;
            sprite.SetOpacity(veil.strength)?;
            sprite.SetIsVisible(visible)?;
        }
        for (layer, panel) in self.panels.iter().zip(&veil.panels) {
            layer.update(panel)?;
        }
        for (layer, panel) in self.flats.iter().zip(&veil.flats) {
            layer.update(panel)?;
        }
        for (layer, divider) in self.dividers.iter().zip(&veil.dividers) {
            layer.update(divider)?;
        }
        for (layer, icon) in self.icons.iter_mut().zip(&veil.icons) {
            layer.update(compositor, painter, icon)?;
        }
        for (layer, text) in self.texts.iter_mut().zip(&veil.texts) {
            layer.update(compositor, painter, text)?;
        }
        Ok(())
    }

    /// Engancha el tinte en degradado a su visual, una sola vez. La forma es
    /// cuadrada; la esquina redondeada la pone el recorte a la ventana.
    fn attach_tint(&self, compositor: &Compositor, veil: &ContainerVisual) -> Result<()> {
        let shape = compositor.CreateSpriteShapeWithGeometry(&self.tint_geometry)?;
        shape.SetFillBrush(&veil_gradient(compositor, self.style.0)?)?;
        self.tint.Shapes()?.Append(&shape)?;
        veil.Children()?.InsertAtTop(&self.tint)?;
        Ok(())
    }

    /// Crea el desenfoque sobre el fondo de la propia ventana, una sola vez.
    fn ensure_blur(
        &mut self,
        compositor: &Compositor,
        veil: &ContainerVisual,
        data: &HeaderVeil,
    ) -> Result<()> {
        let backdrop = compositor.CreateBackdropBrush()?;
        let factory = create_blur_factory(compositor)?;
        let brush = factory.CreateBrush()?;
        brush.SetSourceParameter(&HSTRING::from(SOURCE_NAME), &backdrop)?;
        brush
            .Properties()?
            .InsertScalar(&HSTRING::from(BLUR_ANIMATABLE), data.blur_radius)?;
        let sprite = compositor.CreateSpriteVisual()?;
        sprite.SetBrush(&brush)?;
        sprite.SetIsHitTestVisible(false)?;
        sprite.SetIsVisible(false)?;
        veil.Children()?.InsertAtTop(&sprite)?;
        self.blur_sprite = Some(sprite);
        Ok(())
    }

    /// Escribe el fallo del desenfoque una sola vez en consola.
    fn warn_once(&mut self, detail: &str) {
        if !self.warned {
            self.warned = true;
            eprintln!("[dokka-win-composition] franja de Preferencias sin desenfoque ({detail})");
        }
    }
}

/// `true` si la franja reutiliza sus capas: ambas ausentes o misma forma.
pub(crate) fn veil_matches(current: &Option<HeaderVeilLayer>, next: &Option<HeaderVeil>) -> bool {
    match (current, next) {
        (Some(layer), Some(veil)) => layer.matches(veil),
        (None, None) => true,
        _ => false,
    }
}

/// Lo que obliga a reconstruir la franja si cambia.
fn veil_style_of(veil: &HeaderVeil) -> (dokka_scene::Color, u32) {
    (veil.tint, veil.blur_radius.to_bits())
}
