use crate::ContractError;

/// La papelera de reciclaje del sistema.
pub trait RecycleBin {
    /// Vacía la papelera con la confirmación propia de Windows; nunca sin preguntar.
    /// Cancelar no es un error.
    fn empty(&self) -> Result<(), ContractError>;
}
