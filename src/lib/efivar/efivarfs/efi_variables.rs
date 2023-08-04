use crate::efi_variable_attributes::parse_attributes;
use crate::types::{EfiGuid, EfiVariable};
use crate::MIN_VAR_FILE_NAME_LEN;
use std::fs::{self, ReadDir};
use std::io;
use std::path::PathBuf;

const EFIVARFS_PATH: &'static str = "/sys/firmware/efi/efivars";

pub struct EfiVariables {
    path: PathBuf,
}

pub struct EfiVariablesNameIter {
    dir_entry_iter: Option<ReadDir>,
}

fn convert_name(name: Option<&str>) -> Result<String, String> {
    return match name {
        Some(n) => {
            if n.len() < MIN_VAR_FILE_NAME_LEN {
                return Err(format!(
                    "file name {n} does not represent an EFI variable name"
                ));
            }
            if n.bytes().nth(n.len() - MIN_VAR_FILE_NAME_LEN + 1).unwrap() != b'-' {
                return Err(format!(
                    "file name {n} does not represent an EFI variable name"
                ));
            }
            let guid_bytes = &n[n.len() - MIN_VAR_FILE_NAME_LEN + 2..n.len()];
            match EfiGuid::try_from(guid_bytes) {
                Ok(_) => (),
                Err(_) => {
                    return Err(format!(
                        "file name {n} does not represent an EFI variable name"
                    ))
                }
            };
            let suffix = &n[0..n.len() - MIN_VAR_FILE_NAME_LEN + 1];

            return Ok(String::new() + guid_bytes + &"-" + suffix);
        }
        None => Err("no name provided".to_string()),
    };
}

impl Iterator for EfiVariablesNameIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        return match &mut self.dir_entry_iter {
            Some(iter) => {
                for entry in iter {
                    let converted_name = match entry {
                        Ok(entry) => convert_name(entry.file_name().to_str()),
                        Err(e) => Err(e.to_string()),
                    };
                    if converted_name.is_ok() {
                        return converted_name.ok();
                    }
                }
                return None;
            }
            None => None,
        };
    }
}

impl Default for EfiVariables {
    fn default() -> Self {
        return EfiVariables {
            path: EFIVARFS_PATH.into(),
        };
    }
}

impl EfiVariables {
    pub fn set_path(&mut self, path: PathBuf) -> &EfiVariables {
        self.path = path;
        return self;
    }

    pub fn list(&mut self) -> io::Result<EfiVariablesNameIter> {
        return match fs::metadata(self.path.as_path()) {
            Ok(m) => {
                if m.is_dir() {
                    let iter = fs::read_dir(&self.path);
                    return Ok(EfiVariablesNameIter {
                        dir_entry_iter: match iter {
                            Ok(i) => Some(i),
                            Err(_) => None,
                        },
                    });
                }
                /*
                 * Should return NotADirectory, but Rust doesn't support that, so return NotFound
                 * instead.
                 */
                return Err(io::ErrorKind::NotFound.into());
            }
            Err(e) => Err(e),
        };
    }

    pub fn get_variable(&self, name: &str) -> io::Result<EfiVariable> {
        if name.len() < MIN_VAR_FILE_NAME_LEN
            || name.bytes().nth(MIN_VAR_FILE_NAME_LEN - 2).unwrap() != b'-'
        {
            return Err(io::ErrorKind::InvalidInput.into());
        }
        let guid_bytes = &name[0..MIN_VAR_FILE_NAME_LEN - 2];
        let guid = match EfiGuid::try_from(guid_bytes) {
            Ok(g) => Some(g),
            Err(_) => {
                return Err(io::ErrorKind::InvalidInput.into());
            }
        }
        .unwrap();
        let prefix = &name[MIN_VAR_FILE_NAME_LEN - 1..];
        let full_path = self.path.join(String::new() + prefix + &"-" + guid_bytes);
        let bytes: Vec<u8> = match fs::read(full_path) {
            Ok(bytes) => bytes,
            Err(e) => return Err(e),
        };
        if bytes.len() < 4 {
            return Err(io::ErrorKind::InvalidData.into());
        }
        let attrs = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        return Ok(EfiVariable {
            name: name[MIN_VAR_FILE_NAME_LEN - 1..].into(),
            attributes: parse_attributes(attrs).into(),
            data: bytes[4..].to_vec(),
            guid,
        });
    }
}
