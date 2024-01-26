use crate::holon_errors::HolonError;
use crate::holon_types::Holon;
use hdk::prelude::*;

pub trait HolonReferenceFns {
    fn get_holon(&self) -> Result<Holon, HolonError>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum HolonReference {
    Local(LocalHolonReference),
    // External(ExternalHolonReference),
}

impl HolonReferenceFns for HolonReference {
    fn get_holon(&self) -> Result<Holon, HolonError> {
        match self {
            HolonReference::Local(holon_reference) => holon_reference.get_holon(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct LocalHolonReference {
    holon_id: Option<ActionHash>,
    holon: Option<Holon>,
}

impl HolonReferenceFns for LocalHolonReference {
    /// get_holon will return the cached Holon, first retrieving it from the storage tier, if necessary
    fn get_holon(&self) -> Result<Holon, HolonError> {
        let holon_reference = self.clone();
        if let Some(holon) = holon_reference.holon {
            Ok(holon)
        } else {
            if let Some(id) = holon_reference.holon_id {
                Holon::fetch_holon(id)
            } else {
                Err(HolonError::HolonNotFound(
                    "LocalHolonReference is empty".to_string(),
                ))
            }
        }
    }
}

impl LocalHolonReference {
    pub fn new() -> LocalHolonReference {
        LocalHolonReference {
            holon_id: None,
            holon: None,
        }
    }
    pub fn with_holon(&mut self, holon: Holon) -> &mut Self {
        self.holon = Some(holon);
        self
    }
}