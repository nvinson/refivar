use crate::efi_guids_list_path;
use crate::types::EfiGuid;
use crate::types::EfiGuidListEntry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Error};

pub const DEFAULT_GUIDS_LIST_PATH: &str = efi_guids_list_path::VALUE;

pub enum GuidListSortField {
    Guid,
    Id,
    None,
}

pub struct EfiGuidList {
    guids_map: Option<HashMap<String, EfiGuidListEntry>>,
}

impl Default for EfiGuidList {
    fn default() -> Self {
        Self::new()
    }
}

impl EfiGuidList {
    pub fn new() -> Self {
        Self { guids_map: None }
    }

    pub fn load(&mut self, path: &String) -> Result<(), Error> {
        let mut map: HashMap<String, EfiGuidListEntry> = HashMap::new();
        let reader = BufReader::new(File::open(path)?);
        let result: serde_json::Result<Vec<EfiGuidListEntry>> = serde_json::from_reader(reader);
        match result {
            Ok(v) => {
                for entry in v {
                    map.insert(
                        entry.name.clone(),
                        EfiGuidListEntry {
                            guid: entry.guid,
                            name: entry.name.clone(),
                            description: entry.description,
                        },
                    );
                }
            }
            Err(e) => return Err(e.into()),
        }
        map.insert(
            "zero".to_string(),
            EfiGuidListEntry {
                guid: EfiGuid::from(&[0u8; 16]),
                name: "zero".to_string(),
                description: "zeroed sentinel guid".to_string(),
            },
        );

        self.guids_map = Some(map);
        Ok(())
    }

    pub fn guids(&self, sorted_by: GuidListSortField) -> Vec<&EfiGuidListEntry> {
        match sorted_by {
            GuidListSortField::None => self
                .guids_map
                .as_ref()
                .unwrap()
                .values()
                .collect::<Vec<_>>(),
            GuidListSortField::Guid => {
                let mut sorted_guids = self
                    .guids_map
                    .as_ref()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>();
                sorted_guids.sort_unstable_by(|e1, e2| e1.guid.cmp(&e2.guid));
                sorted_guids
            }
            GuidListSortField::Id => {
                let mut sorted_guids = self
                    .guids_map
                    .as_ref()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>();
                sorted_guids.sort_unstable_by(|e1, e2| e1.name.cmp(&e2.name));
                sorted_guids
            }
        }
    }
}
