use dokka_scene::Scene;
use dokka_win_surface::SurfaceHandle;
use windows::System::DispatcherQueueController;
use windows::UI::Composition::Desktop::DesktopWindowTarget;
use windows::UI::Composition::{CompositionGraphicsDevice, Compositor, ContainerVisual};
use windows::Win32::System::WinRT::Composition::ICompositorDesktopInterop;
use windows::Win32::System::WinRT::{
    CreateDispatcherQueueController, DQTAT_COM_STA, DQTYPE_THREAD_CURRENT, DispatcherQueueOptions,
};
use windows::core::{Interface, Result};
use windows_numerics::Vector3;

use crate::CompositionError;
use crate::graphics_device::create_graphics_device;
use crate::scene_layers::SceneLayers;

/// Mantiene vivos los objetos que sostienen la imagen en pantalla.
pub struct SceneRenderer {
    layers: Option<SceneLayers>,
    root: ContainerVisual,
    device: CompositionGraphicsDevice,
    compositor: Compositor,
    _target: DesktopWindowTarget,
    // Va al final: el compositor necesita la cola mientras exista.
    _queue: DispatcherQueueController,
}

impl SceneRenderer {
    /// Prepara el dibujo en la ventana. Se llama una vez, desde el hilo de la ventana.
    pub fn attach(
        surface: SurfaceHandle,
        scale: f32,
    ) -> std::result::Result<Self, CompositionError> {
        Ok(Self::build(surface, scale)?)
    }

    /// Muestra la escena. Si las imágenes no cambian, solo recoloca lo existente.
    pub fn draw(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        Ok(self.apply(scene)?)
    }

    fn build(surface: SurfaceHandle, scale: f32) -> Result<Self> {
        let queue = create_queue()?;
        let compositor = Compositor::new()?;
        let interop: ICompositorDesktopInterop = compositor.cast()?;
        // SAFETY: el handle viene de una `Surface` creada en este mismo hilo.
        let target = unsafe { interop.CreateDesktopWindowTarget(surface.raw(), true)? };

        let root = compositor.CreateContainerVisual()?;
        root.SetScale(Vector3 {
            X: scale,
            Y: scale,
            Z: 1.0,
        })?;
        target.SetRoot(&root)?;

        Ok(Self {
            layers: None,
            device: create_graphics_device(&compositor)?,
            root,
            compositor,
            _target: target,
            _queue: queue,
        })
    }

    fn apply(&mut self, scene: &Scene) -> Result<()> {
        let layers = match self.layers.take() {
            Some(layers) if layers.matches(scene) => layers,
            _ => SceneLayers::build(&self.compositor, &self.device, &self.root, scene)?,
        };
        layers.update(scene)?;
        self.layers = Some(layers);
        Ok(())
    }
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
