//! Copia desenfocada del contenido durante el zoom del Launchpad.
//!
//! Captura el contenedor `content` con un `CompositionVisualSurface` y lo pinta
//! con un pincel de desenfoque gaussiano sobre un `SpriteVisual` hermano, encima
//! del contenido. El contenido cuelga de un `holder` propio: con desenfoque
//! activo el `holder` se oculta y solo se ve la copia, así no hay doble imagen,
//! y la captura no pierde nitidez porque es del `content`. En reposo el
//! `holder` se muestra y el contenido se ve directo, sin efectos activos. La
//! fábrica se usa una sola vez para crear el pincel, que se reutiliza; si no se
//! pueden crear, se sigue con el zoom y el desvanecido clásicos sin error visible.

use dokka_scene::Scene;
use windows::UI::Composition::{
    CompositionEffectBrush, CompositionVisualSurface, Compositor, ContainerVisual, SpriteVisual,
};
use windows::core::{HSTRING, Result};
use windows_numerics::{Vector2, Vector3};

use crate::blur_effect::{BLUR_ANIMATABLE, SOURCE_NAME, create_blur_factory};

/// Por debajo de este radio se considera reposo y se oculta la copia.
const BLUR_ACTIVE_MIN: f32 = 0.01;

/// Versión desenfocada del contenido, creada de forma perezosa.
pub(crate) struct BlurStage {
    brush: Option<CompositionEffectBrush>,
    surface: Option<CompositionVisualSurface>,
    sprite: Option<SpriteVisual>,
    /// `true` si el efecto no se pudo crear: no se reintenta.
    unsupported: bool,
}

impl BlurStage {
    pub(crate) fn new() -> Self {
        Self {
            brush: None,
            surface: None,
            sprite: None,
            unsupported: false,
        }
    }

    /// Aplica opacidad, escala y desenfoque. En reposo muestra el `holder` y deja
    /// el contenido normal con la copia oculta; en transición oculta el
    /// `holder`, deja el contenido como fuente limpia (opaco y sin escalar) y
    /// la copia visible lleva el desvanecido y la escala de la escena.
    pub(crate) fn apply(
        &mut self,
        compositor: &Compositor,
        root: &ContainerVisual,
        backdrop: &ContainerVisual,
        holder: &ContainerVisual,
        content: &ContainerVisual,
        scene: &Scene,
    ) -> Result<()> {
        let bounds = scene.panel.bounds;
        let center = Vector3 {
            X: bounds.x + bounds.width / 2.0,
            Y: bounds.y + bounds.height / 2.0,
            Z: 0.0,
        };
        backdrop.SetOpacity(scene.opacity)?;
        if scene.blur <= BLUR_ACTIVE_MIN {
            if let Some(sprite) = &self.sprite {
                sprite.SetIsVisible(false)?;
            }
            holder.SetOpacity(1.0)?;
            content.SetOpacity(scene.opacity)?;
            content.SetScale(Vector3 {
                X: scene.scale,
                Y: scene.scale,
                Z: 1.0,
            })?;
            return content.SetCenterPoint(center);
        }
        if self.unsupported {
            return self.apply_classic(holder, content, scene, center);
        }
        if self.brush.is_none() && self.ensure(compositor, root, content).is_err() {
            self.unsupported = true;
            return self.apply_classic(holder, content, scene, center);
        }
        let (Some(surface), Some(brush), Some(sprite)) = (&self.surface, &self.brush, &self.sprite)
        else {
            self.unsupported = true;
            return self.apply_classic(holder, content, scene, center);
        };
        surface.SetSourceSize(Vector2 {
            X: bounds.width,
            Y: bounds.height,
        })?;
        brush
            .Properties()?
            .InsertScalar(&HSTRING::from(BLUR_ANIMATABLE), scene.blur)?;
        // El `holder` oculta el contenido nítido bajo la copia; la captura es
        // del `content`, así que no le afecta y sigue limpia. La copia visible
        // lleva el desvanecido y la escala de la escena.
        holder.SetOpacity(0.0)?;
        content.SetOpacity(1.0)?;
        content.SetScale(Vector3 {
            X: 1.0,
            Y: 1.0,
            Z: 1.0,
        })?;
        content.SetCenterPoint(center)?;
        sprite.SetSize(Vector2 {
            X: bounds.width,
            Y: bounds.height,
        })?;
        sprite.SetOffset(Vector3 {
            X: bounds.x,
            Y: bounds.y,
            Z: 0.0,
        })?;
        sprite.SetCenterPoint(center)?;
        sprite.SetScale(Vector3 {
            X: scene.scale,
            Y: scene.scale,
            Z: 1.0,
        })?;
        sprite.SetOpacity(scene.opacity)?;
        sprite.SetIsHitTestVisible(false)?;
        sprite.SetIsVisible(true)
    }

    /// Zoom y desvanecido sin desenfoque, como antes del efecto.
    fn apply_classic(
        &self,
        holder: &ContainerVisual,
        content: &ContainerVisual,
        scene: &Scene,
        center: Vector3,
    ) -> Result<()> {
        if let Some(sprite) = &self.sprite {
            sprite.SetIsVisible(false)?;
        }
        holder.SetOpacity(1.0)?;
        content.SetOpacity(scene.opacity)?;
        content.SetScale(Vector3 {
            X: scene.scale,
            Y: scene.scale,
            Z: 1.0,
        })?;
        content.SetCenterPoint(center)
    }

    /// Crea superficie, pinceles y copia una sola vez.
    fn ensure(
        &mut self,
        compositor: &Compositor,
        root: &ContainerVisual,
        content: &ContainerVisual,
    ) -> Result<()> {
        let surface = compositor.CreateVisualSurface()?;
        surface.SetSourceVisual(content)?;
        let surface_brush = compositor.CreateSurfaceBrushWithSurface(&surface)?;
        let factory = create_blur_factory(compositor)?;
        let brush = factory.CreateBrush()?;
        brush.SetSourceParameter(&HSTRING::from(SOURCE_NAME), &surface_brush)?;
        brush
            .Properties()?
            .InsertScalar(&HSTRING::from(BLUR_ANIMATABLE), BLUR_ACTIVE_MIN)?;
        let sprite = compositor.CreateSpriteVisual()?;
        sprite.SetBrush(&brush)?;
        sprite.SetIsHitTestVisible(false)?;
        sprite.SetIsVisible(false)?;
        root.Children()?.InsertAtTop(&sprite)?;
        self.surface = Some(surface);
        self.brush = Some(brush);
        self.sprite = Some(sprite);
        Ok(())
    }
}
