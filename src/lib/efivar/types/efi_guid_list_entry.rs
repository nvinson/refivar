use crate::types::efi_guid::EfiGuid;
use serde::de::{Deserialize};
use std::fmt;

#[derive(PartialEq)]
pub struct EfiGuidListEntry {
    pub guid: EfiGuid,
    pub name: String,
    pub description: String,
}

impl<'de> Deserialize<'de> for EfiGuidListEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl fmt::Display for EfiGuidListEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{{}}}\t{{{}}}\t{}",
            self.guid, self.name, self.description
        )
    }
}
