use std::sync::Arc;

use dokka_contracts::{ContractError, SystemTray};
use dokka_model::{TrayItem, TrayItemId};
use dokka_scene::Bitmap;

/// Escucha la bandeja mientras vive; al soltarlo Windows vuelve a atenderla solo.
#[derive(Debug)]
pub struct TrayHost {}

impl TrayHost {
    pub fn start() -> Result<Self, ContractError> {
        Err(ContractError::Failed("bandeja sin implementar".to_owned()))
    }

    /// Cambia cada vez que cambian los iconos, sus textos o sus imágenes.
    pub fn revision(&self) -> u64 {
        0
    }

    /// Imagen del icono; el mismo `Arc` mientras no cambie.
    pub fn icon(&self, _id: TrayItemId) -> Option<Arc<Bitmap>> {
        None
    }
}

impl SystemTray for TrayHost {
    fn items(&self) -> Vec<TrayItem> {
        Vec::new()
    }

    fn activate(&self, _id: TrayItemId) -> Result<(), ContractError> {
        Err(ContractError::NotFound)
    }

    fn open_menu(&self, _id: TrayItemId) -> Result<(), ContractError> {
        Err(ContractError::NotFound)
    }
}
