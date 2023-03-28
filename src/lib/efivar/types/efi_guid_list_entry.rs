use super::efi_guid::EfiGuid;
use std::fmt;

pub struct EfiGuidListEntry {
    pub guid: EfiGuid,
    pub name: &'static str,
    pub description: &'static str,
}

impl fmt::Display for EfiGuidListEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{{{}}}\t{{{}}}\tefi_guid_{}\t{}\n",
            self.guid, self.name, self.name, self.description
        );
    }
}
