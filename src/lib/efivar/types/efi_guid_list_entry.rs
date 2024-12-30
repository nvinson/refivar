use crate::types::efi_guid::EfiGuid;
use crate::types::EfiGuidError;
use serde::de::{Deserialize, Error, MapAccess, Visitor};
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq)]
pub struct EfiGuidListEntry {
    pub guid: EfiGuid,
    pub name: String,
    pub description: String,
}

struct EfiGuidListEntryVisitor {}

impl EfiGuidListEntryVisitor {
    fn new() -> Self {
        EfiGuidListEntryVisitor{}
    }
}

impl<'de> Visitor<'de> for EfiGuidListEntryVisitor {
    type Value = EfiGuidListEntry;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("EfiGuiListEntry object")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut guid: Option<Result<EfiGuid, EfiGuidError>> = None;
        let mut name: Option<String> = None;
        let mut description: Option<String> = None;

        while let Some((key, value)) = access.next_entry::<String, String>()? {
            if key == "description" {
                description = Some(value);
            } else if key == "guid" {
                guid = Some(EfiGuid::from_str(&value));
            } else if key ==  "name" {
                name = Some(value);
            } else {
                eprintln!("Unknown key: {} with value {}", key, value)
            }
        }

        if description.is_none() {
            return Err(M::Error::custom(&"description missing"));
        };
        if guid.is_none() {
            return Err(M::Error::custom(&"guid missing"));
        };
        if name.is_none() {
            return Err(M::Error::custom(&"name missing"));
        };

        let efi_guid_result = guid.unwrap();
        if efi_guid_result.is_err() {
            return Err(M::Error::custom(efi_guid_result.unwrap_err()));
        }
        Ok(EfiGuidListEntry{
            description: description.unwrap(),
            guid: efi_guid_result.unwrap(),
            name: name.unwrap(),
        })
    }
}

impl<'de> Deserialize<'de> for EfiGuidListEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(EfiGuidListEntryVisitor::new())
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
