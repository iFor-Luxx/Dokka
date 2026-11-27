use dokka_scene::{Controls, Scene};
use dokka_win_surface::SurfaceHandle;
use windows::System::DispatcherQueueController;
use windows::UI::Composition::Desktop::DesktopWindowTarget;
use windows::UI::Composition::{Compositor, ContainerVisual};
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DQTAT_COM_STA, DQTYPE_THREAD_CURRENT, DispatcherQueueOptions,
};
use windows::core::{Interface, Result};
use windows_numerics::Vector3;

use crate::CompositionError;
use crate::controls_layer::ControlsLayer;
use crate::large_preview::LargePreview;
use crate::painter::Painter;
use crate::scene_layers::SceneLayers;
use crate::window_thumbnails::WindowThumbnails;

/// Árbol visual enlazado a una ventana.
struct WindowRoot {
    root: ContainerVisual,
    _target: DesktopWindowTarget,
}

/// Ventanas que dibuja el compositor, cada una en su árbol visual.
pub struct WindowHandles {
    pub main: SurfaceHandle,
    pub above: SurfaceHandle,
    pub launchpad: SurfaceHandle,
    /// Vista grande, por debajo del dock.
    pub preview: SurfaceHandle,
    pub settings: SurfaceHandle,
    pub menubar: SurfaceHandle,
}

/// Mantiene vivos los objetos que sostienen la imagen en pantalla.
pub struct SceneRenderer {
    thumbnails: WindowThumbnails,
    preview: LargePreview,
    /// El primer fallo de DWM de la vista grande ya se escribió en consola.
    preview_error_logged: bool,
    layers: Option<SceneLayers>,
    launchpad_layers: Option<SceneLayers>,
    settings_layers: Option<SceneLayers>,
    menubar_layers: Option<SceneLayers>,
    controls: Option<ControlsLayer>,
    main: WindowRoot,
    above: WindowRoot,
    launchpad: WindowRoot,
    /// Objetivo propio de la vista grande: contenedor vacío, sin fondo. Sin él
    /// DWM no pinta nada en una ventana con `WS_EX_NOREDIRECTIONBITMAP`.
    /// Solo se conserva vivo; nunca se dibuja en él.
    _preview_root: WindowRoot,
    settings: WindowRoot,
    menubar: WindowRoot,
    painter: Painter,
    compositor: Compositor,
    // Va al final: el compositor necesita la cola mientras exista.
    _queue: DispatcherQueueController,
}

impl SceneRenderer {
    /// Prepara el dibujo desde el hilo de las ventanas, una sola vez. Un solo compositor y
    /// una sola cola sirven a todos los árboles visuales.
    /// `icon_bytes` son los bytes `&'static` de la fuente de iconos en memoria.
    pub fn attach(
        handles: WindowHandles,
        scale: f32,
        settings_scale: f32,
        font_family: &str,
        icon_bytes: &'static [u8],
    ) -> std::result::Result<Self, CompositionError> {
        Ok(Self::build(
            handles,
            scale,
            settings_scale,
            font_family,
            icon_bytes,
        )?)
    }

    /// Muestra una escena en su árbol visual: el dock, el Launchpad a pantalla
    /// completa o Preferencias. Si su forma no cambia, solo actualiza.
    pub fn draw(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        Ok(self.apply(scene)?)
    }

    /// Vista grande en su sitio real. `false` si no hay imagen válida.
    /// El primer fallo de DWM se escribe una vez en consola.
    pub fn show_large(&mut self, source: u64, area: [i32; 4]) -> bool {
        match self.preview.show(source, area) {
            Ok(valid) => valid,
            Err(error) => {
                if !self.preview_error_logged {
                    self.preview_error_logged = true;
                    eprintln!("vista grande: {error}");
                }
                false
            }
        }
    }

    /// Esconde la vista grande.
    pub fn hide_large(&mut self) {
        self.preview.hide();
    }

    /// Muestra el Launchpad a pantalla completa en su propio árbol visual.
    pub fn draw_launchpad(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        let layers = self.launchpad_layers.take();
        self.launchpad_layers = Some(Self::apply_tree(
            &self.compositor,
            &self.painter,
            &self.launchpad.root,
            layers,
            scene,
        )?);
        Ok(())
    }
    pub fn draw_settings(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        let layers = self.settings_layers.take();
        self.settings_layers = Some(Self::apply_tree(
            &self.compositor,
            &self.painter,
            &self.settings.root,
            layers,
            scene,
        )?);
        Ok(())
    }

    /// Dibuja la barra superior en su propio árbol visual. Usa la misma escala
    /// que el dock: ambas ventanas cubren el monitor principal.
    pub fn draw_menubar(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        let layers = self.menubar_layers.take();
        self.menubar_layers = Some(Self::apply_tree(
            &self.compositor,
            &self.painter,
            &self.menubar.root,
            layers,
            scene,
        )?);
        Ok(())
    }

    fn build(
        handles: WindowHandles,
        scale: f32,
        settings_scale: f32,
        font_family: &str,
        icon_bytes: &'static [u8],
    ) -> Result<Self> {
        let queue = create_queue()?;
        let compositor = Compositor::new()?;
        Ok(Self {
            thumbnails: WindowThumbnails::new(handles.main.raw(), scale),
            preview: LargePreview::new(handles.preview.raw()),
            preview_error_logged: false,
            layers: None,
            launchpad_layers: None,
            settings_layers: None,
            menubar_layers: None,
            controls: None,
            main: window_root(&compositor, handles.main, scale)?,
            above: window_root(&compositor, handles.above, scale)?,
            launchpad: window_root(&compositor, handles.launchpad, scale)?,
            _preview_root: window_root(&compositor, handles.preview, 1.0)?,
            settings: window_root(&compositor, handles.settings, settings_scale)?,
            menubar: window_root(&compositor, handles.menubar, scale)?,
            painter: Painter::new(&compositor, font_family, icon_bytes, scale)?,
            compositor,
            _queue: queue,
        })
    }

    fn apply(&mut self, scene: &Scene) -> Result<()> {
        let mut layers = match self.layers.take() {
            Some(layers) if layers.matches(scene) => layers,
            _ => SceneLayers::build(&self.compositor, &self.painter, &self.main.root, scene)?,
        };
        layers.update(&self.compositor, &self.painter, scene)?;
        self.layers = Some(layers);

        let overlay = scene.overlay.as_ref();
        self.apply_controls(overlay.and_then(|overlay| overlay.controls.as_ref()))?;

        let (floating, opacity, floating_scale, origin) = overlay
            .map(|overlay| {
                (
                    overlay.thumbnails.as_slice(),
                    overlay.opacity,
                    overlay.scale,
                    overlay.origin,
                )
            })
            .unwrap_or((&[], 1.0, 1.0, (0.0, 0.0)));
        self.thumbnails
            .show(&scene.thumbnails, floating, opacity, floating_scale, origin)
    }

    fn apply_controls(&mut self, controls: Option<&Controls>) -> Result<()> {
        let Some(controls) = controls else {
            self.controls = None;
            return self.above.root.Children()?.RemoveAll();
        };
        let mut layer = match self.controls.take() {
            Some(layer) if layer.matches(controls) => layer,
            _ => ControlsLayer::add(&self.compositor, &self.painter, &self.above.root, controls)?,
        };
        layer.update(&self.compositor, &self.painter, controls)?;
        self.controls = Some(layer);
        Ok(())
    }

    /// Reutiliza las capas si la forma coincide; si no, las reconstruye.
    fn apply_tree(
        compositor: &Compositor,
        painter: &Painter,
        root: &ContainerVisual,
        layers: Option<SceneLayers>,
        scene: &Scene,
    ) -> Result<SceneLayers> {
        let mut layers = match layers {
            Some(layers) if layers.matches(scene) => layers,
            _ => SceneLayers::build(compositor, painter, root, scene)?,
        };
        layers.update(compositor, painter, scene)?;
        Ok(layers)
    }
}

fn window_root(compositor: &Compositor, surface: SurfaceHandle, scale: f32) -> Result<WindowRoot> {
    let interop: ICompositorDesktopInterop = compositor.cast()?;
    // SAFETY: el handle viene de una ventana creada en este mismo hilo.
    let target = unsafe { interop.CreateDesktopWindowTarget(surface.raw(), true)? };
    let root = compositor.CreateContainerVisual()?;
    root.SetScale(Vector3 {
        X: scale,
        Y: scale,
        Z: 1.0,
    })?;
    target.SetRoot(&root)?;
    Ok(WindowRoot {
        root,
        _target: target,
    })
}

fn create_queue() -> Result<DispatcherQueueController> {
    let options = DispatcherQueueOptions {
        dwSize: size_of::<DispatcherQueueOptions>() as u32,
        threadType: DQTYPE_THREAD_CURRENT,
        apartmentType: DQTAT_COM_STA,
    };
    // SAFETY: `options` está inicializada y declara su propio tamaño.
    unsafe { CreateDispatcherQueueController(options) }
}
