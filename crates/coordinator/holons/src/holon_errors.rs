use hdk::prelude::*;
use thiserror::Error;

#[hdk_entry_helper]
#[derive(Error, Eq, PartialEq,Clone)]
pub enum HolonError {
    #[error("{0} field is missing")]
    EmptyField(String),
    #[error("Holon not found: {0}")]
    HolonNotFound(String),
    #[error("WasmError {0}")]
    WasmError(String),
    #[error("Couldn't convert Record to {0}")]
    RecordConversion(String),
    #[error("Invalid HolonReference, {0}")]
    InvalidHolonReference(String),
    #[error("{0} Not Implemented")]
    NotImplemented(String),
    // #[error("Wrong type: {0}")]
    // TypeError(String),

}

impl From<WasmError> for HolonError {
    fn from(e: WasmError) -> Self {
        HolonError::WasmError(e.to_string())
    }
}

impl Into<WasmError> for HolonError {
    fn into(self) -> WasmError {
        wasm_error!("HolonError {:?}", self.to_string())
    }
}
