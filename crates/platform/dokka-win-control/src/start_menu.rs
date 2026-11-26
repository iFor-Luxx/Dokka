use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput,
    VK_LWIN,
};

/// Abre o cierra el menú Inicio, como pulsar la tecla Windows.
pub fn open_start_menu() {
    let keys = [
        windows_key(KEYBD_EVENT_FLAGS(0)),
        windows_key(KEYEVENTF_KEYUP),
    ];
    // SAFETY: `keys` está completo y se declara el tamaño de cada elemento.
    unsafe { SendInput(&keys, size_of::<INPUT>() as i32) };
}

fn windows_key(flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VK_LWIN,
                dwFlags: flags,
                ..Default::default()
            },
        },
    }
}
