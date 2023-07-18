use crate::types::EfiGuid;
use std::fs::{self, ReadDir};
use std::path::PathBuf;
use std::io;

const EFIVARFS_PATH: &'static str = "/sys/firmware/efi/efivars";

// variable file names have 1 or more characters, a dash, then a UUID (36 characters)
const MIN_VAR_FILE_NAME_LEN: usize = 38;

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
            let guid: Option<EfiGuid> = match guid_bytes.try_into() {
                Ok(g) => Some(g),
                Err(_) => None,
            };
            if guid.is_none() {
                return Err(format!(
                    "file name {n} does not represent an EFI variable name"
                ));
            }
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
            },
            Err(e) => Err(e),
        };
    }
}
