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
use crate::painter::Painter;
use crate::scene_layers::SceneLayers;
use crate::window_thumbnails::WindowThumbnails;

/// Árbol visual enlazado a una ventana.
struct WindowRoot {
    root: ContainerVisual,
    _target: DesktopWindowTarget,
}

/// Mantiene vivos los objetos que sostienen la imagen en pantalla.
pub struct SceneRenderer {
    thumbnails: WindowThumbnails,
    layers: Option<SceneLayers>,
    launchpad_layers: Option<SceneLayers>,
    controls: Option<ControlsLayer>,
    main: WindowRoot,
    above: WindowRoot,
    launchpad: WindowRoot,
    painter: Painter,
    compositor: Compositor,
    // Va al final: el compositor necesita la cola mientras exista.
    _queue: DispatcherQueueController,
}

impl SceneRenderer {
    /// Prepara el dibujo. `above` es la ventana que va sobre las miniaturas.
    /// Se llama una vez, desde el hilo de las ventanas.
    pub fn attach(
        surface: SurfaceHandle,
        above: SurfaceHandle,
        launchpad: SurfaceHandle,
        scale: f32,
        font_family: &str,
    ) -> std::result::Result<Self, CompositionError> {
        Ok(Self::build(surface, above, launchpad, scale, font_family)?)
    }

    /// Muestra la escena. Si su forma no cambia, solo actualiza lo existente.
    pub fn draw(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        Ok(self.apply(scene)?)
    }

    /// Muestra el Launchpad a pantalla completa en su propio árbol visual.
    pub fn draw_launchpad(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        Ok(self.apply_launchpad(scene)?)
    }

    fn build(
        surface: SurfaceHandle,
        above: SurfaceHandle,
        launchpad: SurfaceHandle,
        scale: f32,
        font_family: &str,
    ) -> Result<Self> {
        let queue = create_queue()?;
        let compositor = Compositor::new()?;
        Ok(Self {
            thumbnails: WindowThumbnails::new(surface.raw(), scale),
            layers: None,
            launchpad_layers: None,
            controls: None,
            main: window_root(&compositor, surface, scale)?,
            above: window_root(&compositor, above, scale)?,
            launchpad: window_root(&compositor, launchpad, scale)?,
            painter: Painter::new(&compositor, font_family)?,
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

        let (floating, floating_opacity, floating_scale, floating_origin) = overlay
            .map(|overlay| {
                (
                    overlay.thumbnails.as_slice(),
                    overlay.opacity,
                    overlay.scale,
                    overlay.origin,
                )
            })
            .unwrap_or((&[], 1.0, 1.0, (0.0, 0.0)));
        self.thumbnails.show(
            &scene.thumbnails,
            floating,
            floating_opacity,
            floating_scale,
            floating_origin,
        )
    }

    fn apply_controls(&mut self, controls: Option<&Controls>) -> Result<()> {
        let Some(controls) = controls else {
            self.controls = None;
            return self.above.root.Children()?.RemoveAll();
        };
        let mut layer = match self.controls.take() {
            Some(layer) => layer,
            None => {
                ControlsLayer::add(&self.compositor, &self.painter, &self.above.root, controls)?
            }
        };
        layer.update(&self.compositor, &self.painter, controls)?;
        self.controls = Some(layer);
        Ok(())
    }

    fn apply_launchpad(&mut self, scene: &Scene) -> Result<()> {
        let mut layers = match self.launchpad_layers.take() {
            Some(layers) if layers.matches(scene) => layers,
            _ => SceneLayers::build(&self.compositor, &self.painter, &self.launchpad.root, scene)?,
        };
        layers.update(&self.compositor, &self.painter, scene)?;
        self.launchpad_layers = Some(layers);
        Ok(())
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
