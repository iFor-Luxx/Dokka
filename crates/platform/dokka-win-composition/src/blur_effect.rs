//! Efecto de desenfoque gaussiano para el zoom del Launchpad.
//!
//! Describe un `D2D1GaussianBlur` al compositor mediante una implementación
//! propia de `IGraphicsEffect` + `IGraphicsEffectD2D1Interop`: expone las 3
//! propiedades del efecto (índice 0: `BlurAmount` como Single; índice 1:
//! optimización de velocidad como UInt32; índice 2: modo de borde suave como
//! UInt32) y una sola fuente, el parámetro `source`. Solo `BlurAmount` tiene
//! nombre (`Blur.BlurAmount`) y es animable: el radio se cambia con
//! `Properties().InsertScalar` sin reconstruir nada. La fábrica se crea una
//! sola vez.
//!
//! Módulos y firmas comprobados en `windows` 0.62:
//! - `windows::Graphics::Effects::{IGraphicsEffect, IGraphicsEffectSource}`
//!   (feature `Graphics_Effects`).
//! - `windows::Win32::System::WinRT::Graphics::Direct2D::IGraphicsEffectD2D1Interop`
//!   (feature `Win32_System_WinRT_Graphics_Direct2D`).
//! - `windows::Win32::Graphics::Direct2D::CLSID_D2D1GaussianBlur`
//!   (feature `Win32_Graphics_Direct2D`).
//! - `windows::Foundation::PropertyValue::{CreateSingle, CreateUInt32}`
//!   (feature `Foundation`).

use windows::Foundation::{IPropertyValue, PropertyValue};
use windows::Graphics::Effects::{
    IGraphicsEffect, IGraphicsEffect_Impl, IGraphicsEffectSource, IGraphicsEffectSource_Impl,
};
use windows::UI::Composition::{
    CompositionEffectFactory, CompositionEffectSourceParameter, Compositor,
};
use windows::Win32::Foundation::E_INVALIDARG;
use windows::Win32::Graphics::Direct2D::CLSID_D2D1GaussianBlur;
use windows::Win32::System::WinRT::Graphics::Direct2D::{
    GRAPHICS_EFFECT_PROPERTY_MAPPING, GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT,
    IGraphicsEffectD2D1Interop, IGraphicsEffectD2D1Interop_Impl,
};
use windows::core::{Error, GUID, HSTRING, Interface, Result};
use windows_collections::IIterable;
use windows_core::implement;

/// Nombre de la fuente dentro del efecto.
pub(crate) const SOURCE_NAME: &str = "source";
/// Nombre del efecto: prefijo de las propiedades animables.
pub(crate) const EFFECT_NAME: &str = "Blur";
/// Propiedad animable del radio en el pincel del efecto.
pub(crate) const BLUR_ANIMATABLE: &str = "Blur.BlurAmount";
/// Propiedad del radio dentro del efecto.
const BLUR_AMOUNT_NAME: &str = "BlurAmount";
/// Optimización de velocidad (`D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED`).
/// Mismo radio visible que la equilibrada, pero pensada para animarlo.
const BLUR_OPTIMIZATION_SPEED: u32 = 0;
/// Borde suave (`D2D1_BORDER_MODE_SOFT`).
const BLUR_BORDER_MODE_SOFT: u32 = 0;

/// Desenfoque gaussiano con una sola fuente: solo el radio (índice 0) cambia.
#[implement(IGraphicsEffect, IGraphicsEffectD2D1Interop)]
struct GaussianBlurEffect {
    amount: f32,
    source: IGraphicsEffectSource,
}

impl IGraphicsEffectSource_Impl for GaussianBlurEffect_Impl {}

impl IGraphicsEffect_Impl for GaussianBlurEffect_Impl {
    fn Name(&self) -> Result<HSTRING> {
        Ok(HSTRING::from(EFFECT_NAME))
    }

    fn SetName(&self, _name: &HSTRING) -> Result<()> {
        Ok(())
    }
}

impl IGraphicsEffectD2D1Interop_Impl for GaussianBlurEffect_Impl {
    fn GetEffectId(&self) -> Result<GUID> {
        Ok(CLSID_D2D1GaussianBlur)
    }

    fn GetNamedPropertyMapping(
        &self,
        name: &windows_core::PCWSTR,
        index: *mut u32,
        mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING,
    ) -> Result<()> {
        // SAFETY: el sistema de efectos entrega una cadena terminada en cero
        // válida durante la llamada.
        let requested = unsafe { name.to_string() }.map_err(|_| Error::from(E_INVALIDARG))?;
        if requested != BLUR_AMOUNT_NAME {
            return Err(Error::from(E_INVALIDARG));
        }
        // SAFETY: el sistema garantiza punteros válidos para escribir el índice
        // y el modo de la propiedad pedida.
        unsafe {
            index.write(0);
            mapping.write(GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT);
        }
        Ok(())
    }

    fn GetPropertyCount(&self) -> Result<u32> {
        Ok(3)
    }

    fn GetProperty(&self, index: u32) -> Result<IPropertyValue> {
        if index == 0 {
            PropertyValue::CreateSingle(self.amount)?.cast()
        } else if index == 1 {
            PropertyValue::CreateUInt32(BLUR_OPTIMIZATION_SPEED)?.cast()
        } else if index == 2 {
            PropertyValue::CreateUInt32(BLUR_BORDER_MODE_SOFT)?.cast()
        } else {
            Err(Error::from(E_INVALIDARG))
        }
    }

    fn GetSource(&self, index: u32) -> Result<IGraphicsEffectSource> {
        if index != 0 {
            return Err(Error::from(E_INVALIDARG));
        }
        Ok(self.source.clone())
    }

    fn GetSourceCount(&self) -> Result<u32> {
        Ok(1)
    }
}

/// Crea la fábrica del desenfoque con `Blur.BlurAmount` como propiedad animable.
pub(crate) fn create_blur_factory(compositor: &Compositor) -> Result<CompositionEffectFactory> {
    let parameter = CompositionEffectSourceParameter::Create(&HSTRING::from(SOURCE_NAME))?;
    let graphics_source: IGraphicsEffectSource = parameter.cast()?;
    let effect: IGraphicsEffect = GaussianBlurEffect {
        amount: 0.0,
        source: graphics_source,
    }
    .into();
    let animatable: IIterable<HSTRING> = vec![HSTRING::from(BLUR_ANIMATABLE)].into();
    compositor.CreateEffectFactoryWithProperties(&effect, &animatable)
}
