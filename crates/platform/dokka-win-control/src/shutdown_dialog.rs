use windows::Win32::System::Com::{
    CLSCTX_ALL, COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE, CoCreateInstance, CoInitializeEx,
    CoUninitialize,
};
use windows::Win32::UI::Shell::IShellDispatch;
use windows::core::GUID;

/// Objeto `Shell.Application`: su método `ShutdownWindows` abre el cuadro de
/// apagar, reiniciar y suspender sin ejecutar nada por sí solo.
const SHELL_APPLICATION: GUID = GUID::from_u128(0x13709620_c279_11ce_a49e_444553540000);

/// Abre el cuadro de Windows de apagar, reiniciar o suspender.
/// Nunca apaga sin preguntar: `ShutdownWindows` solo muestra el diálogo.
pub fn show_shutdown_dialog() {
    // SAFETY: inicializa COM en este hilo si aún no lo estaba; el puntero
    // nulo indica opciones por defecto y cada estructura declara su tamaño.
    unsafe {
        let initialized = CoInitializeEx(None, COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE);
        if let Ok(shell) =
            CoCreateInstance::<_, IShellDispatch>(&SHELL_APPLICATION, None, CLSCTX_ALL)
        {
            let _ = shell.ShutdownWindows();
        }
        // `S_OK` (0) significa que lo inició esta llamada; `S_FALSE` que ya
        // estaba iniciado y no debe deshacerse aquí.
        if initialized.0 == 0 {
            CoUninitialize();
        }
    }
}
