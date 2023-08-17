use crate::types::efi_guid::EfiGuid;
use std::fmt;

#[derive(PartialEq)]
pub struct EfiGuidListEntry {
    pub guid: EfiGuid,
    pub name: String,
    pub description: String,
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
