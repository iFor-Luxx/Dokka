use dokka_contracts::{ContractError, KeyboardLanguages};
use dokka_model::KeyboardLanguage;
use windows::Win32::Foundation::{LPARAM, WPARAM};
use windows::Win32::Globalization::{
    GetLocaleInfoW, LOCALE_SENGLISHDISPLAYNAME, LOCALE_SISO3166CTRYNAME, LOCALE_SISO639LANGNAME,
    LOCALE_SLOCALIZEDDISPLAYNAME,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyboardLayout, GetKeyboardLayoutList, HKL};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, PostMessageW, WM_INPUTLANGCHANGEREQUEST,
};

/// Idiomas de teclado instalados en Windows.
///
/// `languages` marca como activo el de la ventana al frente. Limitación
/// conocida: cuando la barra de Dokka toma el foco (al abrir un menú), la
/// ventana al frente es la propia Dokka; la isla debe preguntar antes de
/// abrir el menú si quiere el idioma de la app de detrás.
#[derive(Debug, Default)]
pub struct SystemKeyboard;

impl KeyboardLanguages for SystemKeyboard {
    fn languages(&self) -> Result<Vec<KeyboardLanguage>, ContractError> {
        let active = foreground_layout();
        Ok(layout_list()?
            .into_iter()
            .map(|layout| describe(layout, active))
            .collect())
    }

    fn activate_language(&self, id: u64) -> Result<(), ContractError> {
        let layout = layout_list()?
            .into_iter()
            .find(|layout| layout.0 as u64 == id)
            .ok_or(ContractError::NotFound)?;
        // SAFETY: `PostMessageW` no espera a la app; un handle destruido falla sin más.
        unsafe {
            let foreground = GetForegroundWindow();
            if foreground.is_invalid() {
                return Err(ContractError::NotFound);
            }
            // `wParam` en cero y el `HKL` en `lParam`, como pide el mensaje.
            PostMessageW(
                Some(foreground),
                WM_INPUTLANGCHANGEREQUEST,
                WPARAM(0),
                LPARAM(layout.0 as isize),
            )
            .map_err(|error| ContractError::Failed(error.to_string()))
        }
    }
}

/// Distribuciones instaladas, en el orden de Windows.
fn layout_list() -> Result<Vec<HKL>, ContractError> {
    // SAFETY: sin búfer solo pregunta cuántas hay; 0 o menos si falla.
    let count = unsafe { GetKeyboardLayoutList(None) };
    if count <= 0 {
        return Err(ContractError::Failed("no hay idiomas de teclado".into()));
    }
    let mut layouts = vec![HKL::default(); count as usize];
    // SAFETY: el vector declara su tamaño; Windows no escribe más allá.
    let written = unsafe { GetKeyboardLayoutList(Some(layouts.as_mut_slice())) };
    if written <= 0 {
        return Err(ContractError::Failed(
            "no se pudieron leer los idiomas".into(),
        ));
    }
    layouts.truncate(written as usize);
    Ok(layouts)
}

/// Distribución del hilo de la ventana al frente, si hay alguna válida.
fn foreground_layout() -> Option<HKL> {
    // SAFETY: solo lee qué ventana tiene el foco y de qué hilo es.
    unsafe {
        let foreground = GetForegroundWindow();
        if foreground.is_invalid() {
            return None;
        }
        let thread = GetWindowThreadProcessId(foreground, None);
        if thread == 0 {
            return None;
        }
        let layout = GetKeyboardLayout(thread);
        (!layout.is_invalid()).then_some(layout)
    }
}

/// Ficha de un idioma: código corto para la barra y nombre completo.
fn describe(layout: HKL, active: Option<HKL>) -> KeyboardLanguage {
    let lcid = language_id(layout);
    let code = short_code(lcid);
    let name = display_name(lcid, &code);
    KeyboardLanguage {
        id: layout.0 as u64,
        code,
        name,
        is_active: Some(layout) == active,
    }
}

/// La palabra baja del `HKL` es el idioma; como `LCID` lleva orden 0, vale tal cual.
fn language_id(layout: HKL) -> u32 {
    (layout.0 as usize & 0xFFFF) as u32
}

/// País del idioma ("ES", "US") y, si no hay, idioma ("EN"), en mayúsculas.
fn short_code(lcid: u32) -> String {
    if let Some(country) = locale_string(lcid, LOCALE_SISO3166CTRYNAME)
        && country.len() == 2
    {
        return country.to_uppercase();
    }
    if let Some(language) = locale_string(lcid, LOCALE_SISO639LANGNAME) {
        return language.to_uppercase();
    }
    format!("{lcid:04X}")
}

/// Nombre completo ("Español (Perú)"); si Windows no lo da, el código corto.
fn display_name(lcid: u32, code: &str) -> String {
    locale_string(lcid, LOCALE_SLOCALIZEDDISPLAYNAME)
        .or_else(|| locale_string(lcid, LOCALE_SENGLISHDISPLAYNAME))
        .unwrap_or_else(|| code.to_owned())
}

/// Texto de un campo de idioma o `None` si Windows no lo da.
fn locale_string(lcid: u32, field: u32) -> Option<String> {
    // SAFETY: sin búfer solo pide el tamaño, en caracteres con el nulo final.
    let needed = unsafe { GetLocaleInfoW(lcid, field, None) };
    if needed <= 0 {
        return None;
    }
    let mut buffer = vec![0u16; needed as usize];
    // SAFETY: el búfer declara su tamaño; Windows no escribe más allá.
    if unsafe { GetLocaleInfoW(lcid, field, Some(buffer.as_mut_slice())) } <= 0 {
        return None;
    }
    let text = String::from_utf16_lossy(&buffer);
    let text = text.trim_matches('\0').trim();
    (!text.is_empty()).then(|| text.to_owned())
}
