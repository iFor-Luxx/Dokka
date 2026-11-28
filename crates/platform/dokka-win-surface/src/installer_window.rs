//! Ventana del instalador: crea, centra, muestra, oculta y arrastra.
//!
//! Como Preferencias: ventana normal y activable (sale en la barra de
//! tareas), sin estar siempre encima. El contenido lo dibuja el compositor
//! en su propio árbol visual. La ventana exterior es el contenido más el
//! margen transparente donde cabe la sombra dibujada por la plataforma.

use windows::Win32::Foundation::{HWND, WPARAM};
use windows::Win32::Graphics::Dwm::{DWMWA_USE_HOSTBACKDROPBRUSH, DwmSetWindowAttribute};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::HiDpi::{
    DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2, GetDpiForWindow, SetProcessDpiAwarenessContext,
};
use windows::Win32::UI::Input::KeyboardAndMouse::ReleaseCapture;
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, HTCAPTION, HWND_NOTOPMOST, HWND_TOPMOST, IDC_ARROW, LoadCursorW,
    RegisterClassExW, SW_SHOWNORMAL, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, SendMessageW,
    SetWindowPos, ShowWindow, WM_NCLBUTTONDOWN, WS_EX_APPWINDOW, WS_EX_NOREDIRECTIONBITMAP,
    WS_MINIMIZEBOX, WS_POPUP, WS_SYSMENU,
};
use windows::core::{BOOL, Result, w};

use crate::SurfaceError;
use crate::window_procedure::window_procedure;

const BASE_DPI: f32 = 96.0;

/// Ventana propia del instalador, en el hilo que la crea.
#[derive(Debug)]
pub struct InstallerWindow {
    window: HWND,
    placed: bool,
    scale: f32,
    margin: f32,
}

impl InstallerWindow {
    /// Crea la ventana oculta y sin tamaño, con icono en la barra de tareas.
    pub fn create() -> Result<Self, SurfaceError> {
        Ok(Self {
            window: create_installer_window().map_err(SurfaceError::from)?,
            placed: false,
            scale: 1.0,
            margin: 0.0,
        })
    }

    /// Centra el contenido una vez con el aire de la sombra alrededor.
    pub fn place(
        &mut self,
        content_width: f32,
        content_height: f32,
        margin: f32,
    ) -> Result<(), SurfaceError> {
        if self.placed {
            return Ok(());
        }
        self.scale = place_centered(self.window, content_width, content_height, margin)
            .map_err(SurfaceError::from)?;
        self.margin = margin;
        self.placed = true;
        Ok(())
    }

    /// Muestra la ventana; si ya estaba visible, la trae al frente.
    pub fn show(&mut self) -> Result<(), SurfaceError> {
        // SAFETY: la ventana es propia y sigue viva mientras sea visible u oculta.
        unsafe {
            let _ = ShowWindow(self.window, SW_SHOWNORMAL);
            let _ = SetWindowPos(
                self.window,
                Some(HWND_TOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
            let _ = SetWindowPos(
                self.window,
                Some(HWND_NOTOPMOST),
                0,
                0,
                0,
                0,
                SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
            );
        }
        crate::foreground::foreground_window(self.window);
        Ok(())
    }

    /// Empieza el arrastre nativo: Windows mueve la ventana hasta soltar.
    pub fn begin_drag(&self) {
        // SAFETY: la ventana es propia; el mensaje entra en su bucle modal.
        unsafe {
            let _ = ReleaseCapture();
            let _ = SendMessageW(
                self.window,
                WM_NCLBUTTONDOWN,
                Some(WPARAM(HTCAPTION as usize)),
                None,
            );
        }
    }

    /// Escala de la ventana para pasar a píxeles.
    pub fn scale(&self) -> f32 {
        self.scale.max(1.0)
    }

    /// Aire de la sombra alrededor del contenido, en unidades lógicas.
    pub fn margin(&self) -> f32 {
        self.margin.max(0.0)
    }

    pub(crate) fn window(&self) -> HWND {
        self.window
    }

    /// Handle crudo para el compositor. La ventana sigue siendo del hilo que la creó.
    pub fn raw(&self) -> HWND {
        self.window
    }
}

/// Crea la ventana del instalador: normal y activable, sin borde ni título.
fn create_installer_window() -> Result<HWND> {
    let class_name = w!("DokkaSurface");
    // SAFETY: las cadenas son estáticas y `window_procedure` cumple su firma.
    unsafe {
        let _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
        let instance = GetModuleHandleW(None)?;
        let class = RegisterClassExW(&windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW {
            cbSize: size_of::<windows::Win32::UI::WindowsAndMessaging::WNDCLASSEXW>() as u32,
            lpfnWndProc: Some(window_procedure),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            lpszClassName: class_name,
            ..Default::default()
        });
        let _ = class;
        let window = CreateWindowExW(
            WS_EX_NOREDIRECTIONBITMAP | WS_EX_APPWINDOW,
            class_name,
            w!("Dokka"),
            WS_POPUP | WS_SYSMENU | WS_MINIMIZEBOX,
            0,
            0,
            0,
            0,
            None,
            None,
            Some(instance.into()),
            None,
        )?;
        let enabled = BOOL::from(true);
        DwmSetWindowAttribute(
            window,
            DWMWA_USE_HOSTBACKDROPBRUSH,
            (&raw const enabled).cast(),
            size_of::<BOOL>() as u32,
        )?;
        Ok(window)
    }
}

/// Centra el contenido en el área de trabajo con el aire alrededor.
fn place_centered(
    window: HWND,
    content_width: f32,
    content_height: f32,
    margin: f32,
) -> Result<f32> {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::{
        SPI_GETWORKAREA, SWP_NOACTIVATE, SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS, SetWindowPos,
        SystemParametersInfoW,
    };
    // SAFETY: `window` es válida y la escala sale de su DPI real.
    unsafe {
        let mut work_area = RECT::default();
        SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some((&raw mut work_area).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )?;
        let scale = GetDpiForWindow(window) as f32 / BASE_DPI;
        let content_px = (content_width * scale).round() as i32;
        let content_py = (content_height * scale).round() as i32;
        let margin_px = (margin.max(0.0) * scale).round() as i32;
        let content_x = work_area.left + (work_area.right - work_area.left - content_px) / 2;
        let content_y = work_area.top + (work_area.bottom - work_area.top - content_py) / 2;
        SetWindowPos(
            window,
            None,
            content_x - margin_px,
            content_y - margin_px,
            content_px + margin_px * 2,
            content_py + margin_px * 2,
            SWP_NOACTIVATE,
        )?;
        Ok(scale)
    }
}
