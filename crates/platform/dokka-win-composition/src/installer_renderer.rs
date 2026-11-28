//! Dibujador de UNA sola ventana para el instalador.
//!
//! Reutiliza `Painter` y `SceneLayers` como el resto de Dokka: la foto de
//! fondo va colgada al fondo del contenido (debajo de tarjetas y textos,
//! encima del cristal) y recortada a las esquinas. `SceneRenderer` pide seis
//! ventanas y no sirve aquí. La entrada inicial (fondo y montañas que viajan
//! desenfocadas, fundido de la foto y subida del contenido) la animan los
//! visuales de `installer_entry_layers` con el compositor.

use std::sync::Arc;

use dokka_scene::{Color, IconImage, Scene};
use windows::System::DispatcherQueueController;
use windows::UI::Composition::{Compositor, ContainerVisual, SpriteVisual};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DQTAT_COM_STA, DQTYPE_THREAD_CURRENT, DispatcherQueueOptions,
};
use windows::core::Interface;
use windows_numerics::{Vector2, Vector3};

use crate::CompositionError;
use crate::installer_background::decode_background;
use crate::installer_entry::{EntryTiming, InstallerEntryData, animations_disabled};
use crate::installer_entry_layers::InstallerEntryLayers;
use crate::painter::Painter;
use crate::scene_layers::SceneLayers;
use crate::windows_color::windows_color;

/// Foto y geometría del fondo, como datos (la plataforma no usa el tema).
pub struct InstallerPhoto<'a> {
    pub bytes: &'a [u8],
    pub fallback: Color,
    pub width: f32,
    pub height: f32,
    pub margin: f32,
    pub radius: f32,
}

/// Dibuja la escena del instalador en su única ventana.
pub struct InstallerRenderer {
    photo: SpriteVisual,
    photo_placed: bool,
    entry: Option<InstallerEntryLayers>,
    timing: Option<EntryTiming>,
    layers: Option<SceneLayers>,
    content: ContainerVisual,
    painter: Painter,
    compositor: Compositor,
    // Va al final: el compositor necesita la cola mientras exista.
    _queue: DispatcherQueueController,
}

impl InstallerRenderer {
    /// Prepara el dibujo desde el hilo de la ventana, una sola vez.
    pub fn attach(
        window: HWND,
        scale: f32,
        font_family: &str,
        icon_bytes: &'static [u8],
        photo: &InstallerPhoto<'_>,
        entry: &InstallerEntryData<'_>,
    ) -> std::result::Result<Self, CompositionError> {
        Ok(Self::build(
            window,
            scale,
            font_family,
            icon_bytes,
            photo,
            entry,
        )?)
    }

    /// Muestra una escena: reutiliza las capas si la forma coincide.
    pub fn draw(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        let rebuild = self
            .layers
            .as_ref()
            .is_none_or(|layers| !layers.matches(scene));
        let layers = self.layers.take();
        self.layers = Some(Self::apply_tree(
            &self.compositor,
            &self.painter,
            &self.content,
            layers,
            scene,
        )?);
        if rebuild {
            self.photo_placed = false;
        }
        if !self.photo_placed
            && let Some(layers) = &self.layers
        {
            let content = layers.content_root();
            content.Children()?.InsertAtBottom(&self.photo)?;
            self.photo_placed = true;
            if let (Some(entry), Some(timing)) = (&mut self.entry, &self.timing)
                && (entry.place(&content).is_err()
                    || Self::start_entry(&self.compositor, &*entry, timing, &self.photo, &content)
                        .is_err())
            {
                let _ = Self::drop_entry(&mut self.entry, &content, &self.photo, &content);
                self.timing = None;
                let _ = self.photo.SetOpacity(1.0);
                let _ = content.SetOpacity(1.0);
            }
        }
        Ok(())
    }

    /// `true` mientras la entrada sigue montada (aunque ya terminara de
    /// animar): la app la retira al pasar el tiempo total.
    pub fn entry_running(&self) -> bool {
        self.entry.is_some()
    }

    /// Detiene las animaciones, quita fondo y montañas y deja el estado final
    /// sin efectos activos. Sin entrada, no hace nada.
    pub fn finish_entry(&mut self) -> std::result::Result<(), CompositionError> {
        let Some(layers) = &self.layers else {
            self.entry = None;
            self.timing = None;
            return Ok(());
        };
        let content = layers.content_root();
        if let Some(entry) = self.entry.take()
            && let Err(error) = entry.finish(&content, &self.photo, &content)
        {
            self.timing = None;
            return Err(CompositionError::from(error));
        }
        self.timing = None;
        Ok(())
    }

    /// Arranca la entrada una sola vez tras colgar las capas.
    fn start_entry(
        compositor: &Compositor,
        entry: &InstallerEntryLayers,
        timing: &EntryTiming,
        photo: &SpriteVisual,
        content: &ContainerVisual,
    ) -> windows::core::Result<()> {
        entry.start(compositor, photo, content, timing)
    }

    /// Quita la entrada que no pudo arrancar y restaura lo visible.
    fn drop_entry(
        entry: &mut Option<InstallerEntryLayers>,
        parent: &ContainerVisual,
        photo: &SpriteVisual,
        content: &ContainerVisual,
    ) -> windows::core::Result<()> {
        if let Some(layers) = entry.take() {
            let _ = layers.finish(parent, photo, content);
        }
        Ok(())
    }

    fn build(
        window: HWND,
        scale: f32,
        font_family: &str,
        icon_bytes: &'static [u8],
        photo: &InstallerPhoto<'_>,
        entry: &InstallerEntryData<'_>,
    ) -> windows::core::Result<Self> {
        let queue = create_queue()?;
        let compositor = Compositor::new()?;
        let interop: ICompositorDesktopInterop = compositor.cast()?;
        // SAFETY: el handle viene de la ventana creada en este mismo hilo.
        let target = unsafe { interop.CreateDesktopWindowTarget(window, true)? };
        let root = compositor.CreateContainerVisual()?;
        root.SetScale(Vector3 {
            X: scale,
            Y: scale,
            Z: 1.0,
        })?;
        target.SetRoot(&root)?;
        let content = compositor.CreateContainerVisual()?;
        root.Children()?.InsertAtTop(&content)?;
        let painter = Painter::new(&compositor, font_family, icon_bytes, scale)?;
        let visual = make_photo(&compositor, &painter, photo)?;
        content.Children()?.InsertAtBottom(&visual)?;
        let (entry, timing) = if animations_disabled() {
            (None, None)
        } else {
            match InstallerEntryLayers::build(
                &compositor,
                &painter,
                entry,
                photo.width,
                photo.height,
                photo.margin,
                photo.radius,
            ) {
                Ok(layers) => match layers {
                    Some(layers) => (Some(layers), Some(EntryTiming::from(entry))),
                    None => (None, None),
                },
                Err(_) => (None, None),
            }
        };
        Ok(Self {
            photo: visual,
            photo_placed: true,
            entry,
            timing,
            layers: None,
            content,
            painter,
            compositor,
            _queue: queue,
        })
    }

    /// Reutiliza las capas si la forma coincide; si no, las reconstruye.
    fn apply_tree(
        compositor: &Compositor,
        painter: &Painter,
        root: &ContainerVisual,
        layers: Option<SceneLayers>,
        scene: &Scene,
    ) -> windows::core::Result<SceneLayers> {
        let mut layers = match layers {
            Some(layers) if layers.matches(scene) => layers,
            _ => SceneLayers::build(compositor, painter, root, scene)?,
        };
        layers.update(compositor, painter, scene)?;
        Ok(layers)
    }
}

/// Foto recortada a las esquinas o color crema si no se pudo leer.
fn make_photo(
    compositor: &Compositor,
    painter: &Painter,
    photo: &InstallerPhoto<'_>,
) -> windows::core::Result<SpriteVisual> {
    let visual = compositor.CreateSpriteVisual()?;
    if let Some(bitmap) = decode_background(photo.bytes) {
        let image = IconImage::Pixels(Arc::new(bitmap));
        let surface = painter.paint(&image, photo.width)?;
        visual.SetBrush(&compositor.CreateSurfaceBrushWithSurface(&surface)?)?;
    } else {
        visual.SetBrush(&compositor.CreateColorBrushWithColor(windows_color(photo.fallback))?)?;
    }
    visual.SetSize(Vector2 {
        X: photo.width,
        Y: photo.height,
    })?;
    visual.SetOffset(Vector3 {
        X: photo.margin,
        Y: photo.margin,
        Z: 0.0,
    })?;
    let outline = compositor.CreateRoundedRectangleGeometry()?;
    outline.SetSize(Vector2 {
        X: photo.width,
        Y: photo.height,
    })?;
    outline.SetCornerRadius(Vector2 {
        X: photo.radius,
        Y: photo.radius,
    })?;
    visual.SetClip(&compositor.CreateGeometricClipWithGeometry(&outline)?)?;
    Ok(visual)
}

fn create_queue() -> windows::core::Result<DispatcherQueueController> {
    let options = DispatcherQueueOptions {
        dwSize: size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_STA,
    };
    // SAFETY: `options` está inicializada y declara su propio tamaño.
    unsafe { CreateDispatcherQueueController(options) }
}
