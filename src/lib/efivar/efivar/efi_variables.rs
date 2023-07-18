use crate::efivarfs;
use std::io;
use std::path::PathBuf;

const EFIVARS_PATH: &'static str = "/sys/firmware/efi/vars";

pub struct EfiVariables {
    path: PathBuf,
}

impl Default for EfiVariables {
    fn default() -> Self {
        return EfiVariables {
            path: EFIVARS_PATH.into(),
        };
    }
}

impl EfiVariables {
    pub fn set_path(&mut self, path: PathBuf) -> &EfiVariables {
        self.path = path;
        return self;
    }

    pub fn list(&self) -> io::Result<efivarfs::EfiVariablesNameIter> {
        let mut e: efivarfs::EfiVariables = Default::default();
        e.set_path(self.path.clone());
        return e.list();
    }
}
