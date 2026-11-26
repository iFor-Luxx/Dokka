use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP, SendInput,
    VIRTUAL_KEY, VK_D, VK_LWIN,
};

/// Muestra u oculta el escritorio, como Windows + D.
pub fn show_desktop() {
    send(&[press(VK_LWIN), press(VK_D), release(VK_D), release(VK_LWIN)]);
}

fn send(keys: &[INPUT]) {
    // SAFETY: cada elemento está completo y se declara su tamaño.
    unsafe { SendInput(keys, size_of::<INPUT>() as i32) };
}

fn press(key: VIRTUAL_KEY) -> INPUT {
    keyboard(key, KEYBD_EVENT_FLAGS(0))
}

fn release(key: VIRTUAL_KEY) -> INPUT {
    keyboard(key, KEYEVENTF_KEYUP)
}

fn keyboard(key: VIRTUAL_KEY, flags: KEYBD_EVENT_FLAGS) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: key,
                dwFlags: flags,
                ..Default::default()
            },
        },
    }
}
