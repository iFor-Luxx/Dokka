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
use crate::draw_glass::draw_glass;
use crate::draw_icon::draw_icon;
use crate::draw_tint::draw_tint;
use crate::graphics_device::create_graphics_device;

/// Mantiene vivos los objetos que sostienen la imagen en pantalla.
pub struct SceneRenderer {
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

    /// Sustituye lo que se ve por la escena indicada.
    pub fn draw(&mut self, scene: &Scene) -> std::result::Result<(), CompositionError> {
        Ok(self.redraw(scene)?)
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
            device: create_graphics_device(&compositor)?,
            root,
            compositor,
            _target: target,
            _queue: queue,
        })
    }

    fn redraw(&self, scene: &Scene) -> Result<()> {
        let layers = self.root.Children()?;
        layers.RemoveAll()?;
        layers.InsertAtTop(&draw_glass(&self.compositor, &scene.panel)?)?;
        layers.InsertAtTop(&draw_tint(&self.compositor, &scene.panel)?)?;
        for icon in &scene.icons {
            layers.InsertAtTop(&draw_icon(&self.compositor, &self.device, icon)?)?;
        }
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
