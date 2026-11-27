use windows::Win32::Foundation::{HWND, LPARAM, RECT};
use windows::Win32::UI::Shell::{
    ABE_TOP, ABM_NEW, ABM_QUERYPOS, ABM_REMOVE, ABM_SETPOS, APPBARDATA, SHAppBarMessage,
};
use windows::Win32::UI::WindowsAndMessaging::{
    HWND_TOPMOST, SW_HIDE, SW_SHOWNOACTIVATE, SWP_NOACTIVATE, SWP_SHOWWINDOW, SetWindowPos,
    ShowWindow,
};

use crate::menubar_window::{menubar_dpi_scale, primary_monitor_rect};
use crate::window_procedure::MENUBAR_CALLBACK_MESSAGE;
use crate::{Surface, SurfaceError};

impl Surface {
    /// Reserva `bar_height` arriba (las maximizadas empiezan debajo) y coloca
    /// una ventana de `window_height` para dibujar el menú desplegado debajo.
    /// Medidas lógicas. No registra dos veces: si ya estaba, solo recoloca.
    pub fn menubar_show(
        &mut self,
        bar_height: f32,
        window_height: f32,
    ) -> Result<(), SurfaceError> {
        if self.menubar.is_invalid() {
            return Err(windows::core::Error::from_thread().into());
        }
        self.menubar_scale = menubar_dpi_scale(self.menubar).max(1.0);
        let scale = self.menubar_scale;
        let monitor = primary_monitor_rect(self.menubar)?;
        let bar_px = (bar_height.max(0.0) * scale).round() as i32;
        let window_px = (window_height.max(bar_height).max(0.0) * scale).round() as i32;
        self.register_appbar(&monitor, bar_px)?;
        self.position_appbar(&monitor, bar_px)?;
        // SAFETY: la ventana es propia; se ordena topmost sin activarse.
        unsafe {
            SetWindowPos(
                self.menubar,
                Some(HWND_TOPMOST),
                monitor.left,
                monitor.top,
                monitor.right - monitor.left,
                window_px.max(1),
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            )?;
            let _ = ShowWindow(self.menubar, SW_SHOWNOACTIVATE);
        }
        self.menubar_visible = true;
        // Si ya había una app a pantalla completa, no se recibe de nuevo el
        // aviso: se comprueba al mostrar para no taparla.
        self.menubar_fullscreen = false;
        self.refresh_menubar_fullscreen();
        Ok(())
    }

    /// Oculta la ventana y libera el espacio reservado.
    pub fn menubar_hide(&mut self) {
        self.remove_appbar();
        if !self.menubar.is_invalid() {
            // SAFETY: la ventana es propia.
            unsafe {
                let _ = ShowWindow(self.menubar, SW_HIDE);
            }
        }
        self.menubar_visible = false;
        self.menubar_fullscreen = false;
    }

    /// Identificador de la barra para el compositor.
    pub fn menubar_handle(&self) -> crate::SurfaceHandle {
        crate::SurfaceHandle::new(self.menubar)
    }

    /// Ancho lógico del monitor principal.
    pub fn menubar_width(&self) -> f32 {
        if self.menubar.is_invalid() {
            return 0.0;
        }
        let scale = self.menubar_scale.max(1.0);
        match primary_monitor_rect(self.menubar) {
            Ok(area) => (area.right - area.left) as f32 / scale,
            Err(_) => 0.0,
        }
    }

    /// Escala de la barra (la misma que la del dock en el principal).
    pub fn menubar_scale(&self) -> f32 {
        self.menubar_scale.max(1.0)
    }

    pub(crate) fn menubar_window(&self) -> HWND {
        self.menubar
    }

    /// Mira la ventana al frente y oculta o recupera la barra. El aviso de Windows
    /// no basta: salta con el escritorio o con ventanas propias y no siempre
    /// llega el de salida, así que se comprueba también cada segundo.
    pub(crate) fn refresh_menubar_fullscreen(&mut self) {
        let Ok(monitor) = primary_monitor_rect(self.menubar) else {
            return;
        };
        let covering = is_fullscreen_covering(self.menubar, &monitor);
        if covering != self.menubar_fullscreen {
            self.apply_fullscreen(covering);
        }
    }

    /// Oculta ante pantalla completa sin liberar la reserva; al salir vuelve
    /// a mostrarse solo si seguía visible.
    fn apply_fullscreen(&mut self, entering: bool) {
        self.menubar_fullscreen = entering;
        if self.menubar.is_invalid() || !self.menubar_visible {
            return;
        }
        // SAFETY: la ventana es propia.
        unsafe {
            if entering {
                let _ = ShowWindow(self.menubar, SW_HIDE);
            } else {
                let _ = ShowWindow(self.menubar, SW_SHOWNOACTIVATE);
            }
        }
    }

    fn register_appbar(&mut self, monitor: &RECT, bar_px: i32) -> Result<(), SurfaceError> {
        if self.menubar_registered {
            return Ok(());
        }
        let mut data = appbar_data(self.menubar, monitor, bar_px);
        // SAFETY: `data` está completa y describe la ventana propia.
        let registered = unsafe { SHAppBarMessage(ABM_NEW, &mut data) } != 0;
        if !registered {
            return Err(windows::core::Error::from_thread().into());
        }
        self.menubar_registered = true;
        Ok(())
    }

    fn position_appbar(&self, monitor: &RECT, bar_px: i32) -> Result<(), SurfaceError> {
        let mut data = appbar_data(self.menubar, monitor, bar_px);
        // SAFETY: `data` está completa; el sistema ajusta `rc` a las barras ya registradas.
        unsafe {
            SHAppBarMessage(ABM_QUERYPOS, &mut data);
        }
        // Se conserva el borde superior concedido y la altura pedida.
        data.rc.bottom = data.rc.top + bar_px.max(0);
        // SAFETY: `data` lleva el rectángulo ya consultado.
        unsafe {
            SHAppBarMessage(ABM_SETPOS, &mut data);
        }
        Ok(())
    }

    fn remove_appbar(&mut self) {
        if !self.menubar_registered || self.menubar.is_invalid() {
            self.menubar_registered = false;
            return;
        }
        let mut data = APPBARDATA {
            cbSize: size_of::<APPBARDATA>() as u32,
            hWnd: self.menubar,
            ..Default::default()
        };
        // SAFETY: `data` describe la ventana propia registrada arriba.
        unsafe {
            SHAppBarMessage(ABM_REMOVE, &mut data);
        }
        self.menubar_registered = false;
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        // La recarga en vivo mata el proceso sin pasar por aquí: la reserva puede
        // quedarse hasta que el área se reevalúe; en cierres normales se libera.
        self.remove_appbar();
    }
}

fn appbar_data(window: HWND, monitor: &RECT, bar_px: i32) -> APPBARDATA {
    APPBARDATA {
        cbSize: size_of::<APPBARDATA>() as u32,
        hWnd: window,
        uCallbackMessage: MENUBAR_CALLBACK_MESSAGE,
        uEdge: ABE_TOP,
        rc: RECT {
            left: monitor.left,
            top: monitor.top,
            right: monitor.right,
            bottom: monitor.top + bar_px.max(0),
        },
        lParam: LPARAM(0),
    }
}

/// `true` si la ventana al frente es de otra app y cubre todo el monitor.
/// El escritorio y las ventanas de Dokka (Launchpad, vista grande) no cuentan.
fn is_fullscreen_covering(menubar: HWND, monitor: &RECT) -> bool {
    use windows::Win32::System::Threading::GetCurrentProcessId;
    use windows::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetWindowRect, GetWindowThreadProcessId,
    };
    // SAFETY: consultas de solo lectura; los búferes y punteros viven durante cada llamada.
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground.is_invalid() || foreground == menubar {
            return false;
        }
        let mut owner = 0;
        GetWindowThreadProcessId(foreground, Some(&mut owner));
        if owner == GetCurrentProcessId() {
            return false;
        }
        let mut name = [0_u16; 64];
        let length = GetClassNameW(foreground, &mut name).max(0) as usize;
        let class = String::from_utf16_lossy(&name[..length]);
        if class == "Progman" || class == "WorkerW" {
            return false;
        }
        let mut area = RECT::default();
        if GetWindowRect(foreground, &mut area).is_err() {
            return false;
        }
        area.left <= monitor.left
            && area.top <= monitor.top
            && area.right >= monitor.right
            && area.bottom >= monitor.bottom
    }
}
