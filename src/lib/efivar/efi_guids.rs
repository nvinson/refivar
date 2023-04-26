use crate::types::EfiGuid;
use crate::types::EfiGuidListEntry;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

pub const DEFAULT_GUIDS_LIST_PATH: &'static str = env!("GUIDS_LIST_PATH");

pub enum GuidListSortField {
    Guid,
    Id,
    None,
}

pub struct EfiGuidList {
    guids_map: Option<HashMap<String, EfiGuidListEntry>>,
}

#[derive(Deserialize)]
struct JsonEfiGuidListEntry {
    name: String,
    description: String,
    guid: String,
}

impl Default for EfiGuidList {
    fn default() -> Self {
        EfiGuidList { guids_map: None }
    }
}

impl EfiGuidList {
    pub fn load(&mut self, path: &String) -> () {
        let mut map: HashMap<String, EfiGuidListEntry> = HashMap::new();
        match File::open(path) {
            Ok(h) => {
                let reader = BufReader::new(h);
                let result: serde_json::Result<Vec<JsonEfiGuidListEntry>> =
                    serde_json::from_reader(reader);
                match result {
                    Ok(v) => {
                        for entry in v {
                            match EfiGuid::from_str(&entry.guid) {
                                Ok(g) => {
                                    map.insert(
                                        entry.name.clone(),
                                        EfiGuidListEntry {
                                            guid: g,
                                            name: entry.name.clone(),
                                            description: entry.description,
                                        },
                                    );
                                }
                                Err(_) => {
                                    eprintln!(
                                        "Entry with UUID: {} invalid. Skipping...",
                                        entry.guid
                                    );
                                }
                            };
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read GUIDs list file: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to open GUIDs list file: {}", e);
            }
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
    }

    pub fn guids(&self, sorted_by: GuidListSortField) -> Vec<&EfiGuidListEntry> {
        match sorted_by {
            GuidListSortField::None => {
                let guids = self
                    .guids_map
                    .as_ref()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>();
                return guids;
            }
            GuidListSortField::Guid => {
                let mut sorted_guids = self
                    .guids_map
                    .as_ref()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>();
                sorted_guids.sort_unstable_by(|e1, e2| e1.guid.cmp(&e2.guid));
                return sorted_guids;
            }
            GuidListSortField::Id => {
                let mut sorted_guids = self
                    .guids_map
                    .as_ref()
                    .unwrap()
                    .values()
                    .collect::<Vec<_>>();
                sorted_guids.sort_unstable_by(|e1, e2| e1.name.cmp(&e2.name));
                return sorted_guids;
            }
        }
    }
}
