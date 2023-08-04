use crate::efi_variable_attributes::parse_attributes;
use crate::efivarfs;
use crate::types::{EfiGuid, EfiVariable};
use crate::MIN_VAR_FILE_NAME_LEN;
use std::boxed::Box;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::io::{self, IoSliceMut, Read};
use std::path::{Path, PathBuf};

const EFI_VAR_NAME_LEN: usize = 512;
const EFIVARS_FW_PLATFORM_SZ_PATH: &'static str = "/sys/firmware/efi/fw_platform_size";
const EFIVARS_PATH: &'static str = "/sys/firmware/efi/vars";

pub struct EfiVariables {
    path: PathBuf,
    platform_size: usize,
}

impl Default for EfiVariables {
    fn default() -> Self {
        let mut variables = EfiVariables {
            path: EFIVARS_PATH.into(),
            platform_size: 0,
        };
        variables
            .set_firmware_platform_size(
                EfiVariables::get_firmware_platform_size(EFIVARS_FW_PLATFORM_SZ_PATH.into())
                    .unwrap(),
            )
            .unwrap();
        return variables;
    }
}

type Efi64VariableBuffer = EfiVariableBuffer<8>;
type Efi32VariableBuffer = EfiVariableBuffer<4>;

trait EfiNVariableBuffer {
    fn name(&mut self) -> &mut [u8];
    fn guid(&mut self) -> &mut [u8];
    fn data_size(&mut self) -> &mut [u8];
    fn data(&mut self) -> &mut [u8];
    fn status(&mut self) -> &mut [u8];
    fn attributes(&mut self) -> &mut [u8];
}

struct EfiVariableBuffer<const SIZE: usize> {
    name: [u8; EFI_VAR_NAME_LEN * 2],
    guid: [u8; 16],
    data_size: [u8; SIZE],
    data: [u8; 1024],
    status: [u8; SIZE],
    attributes: [u8; 4],
}

impl<const SIZE: usize> EfiVariableBuffer<SIZE> {
    fn new() -> Self {
        return EfiVariableBuffer {
            name: [0; EFI_VAR_NAME_LEN * 2],
            guid: [0; 16],
            data_size: [0; SIZE],
            data: [0; 1024],
            status: [0; SIZE],
            attributes: [0; 4],
        };
    }
}

impl<const SIZE: usize> TryFrom<fs::File> for EfiVariableBuffer<SIZE> {
    type Error = Box<dyn Error>;

    fn try_from(mut handle: fs::File) -> Result<Self, Self::Error> {
        let mut buffer = Self::new();
        let mut vectors = [
            IoSliceMut::new(&mut buffer.name),
            IoSliceMut::new(&mut buffer.guid),
            IoSliceMut::new(&mut buffer.data_size),
            IoSliceMut::new(&mut buffer.data),
            IoSliceMut::new(&mut buffer.status),
            IoSliceMut::new(&mut buffer.attributes),
        ];
        handle.read_vectored(&mut vectors)?;
        return Ok(buffer);
    }
}

impl<const SIZE: usize> EfiNVariableBuffer for EfiVariableBuffer<SIZE> {
    fn name(&mut self) -> &mut [u8] {
        self.name.as_mut_slice()
    }

    fn guid(&mut self) -> &mut [u8] {
        self.guid.as_mut_slice()
    }

    fn data_size(&mut self) -> &mut [u8] {
        self.data_size.as_mut_slice()
    }

    fn data(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }

    fn status(&mut self) -> &mut [u8] {
        self.status.as_mut_slice()
    }

    fn attributes(&mut self) -> &mut [u8] {
        self.attributes.as_mut_slice()
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

    pub fn get_variable(&self, name: &str) -> Result<EfiVariable, Box<dyn Error>> {
        if name.len() < MIN_VAR_FILE_NAME_LEN
            || name.bytes().nth(MIN_VAR_FILE_NAME_LEN - 2).unwrap() != b'-'
        {
            return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
        }
        let guid_bytes = &name[0..MIN_VAR_FILE_NAME_LEN - 2];
        let guid = match EfiGuid::try_from(guid_bytes) {
            Ok(g) => Some(g),
            Err(_) => {
                return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
            }
        }
        .unwrap();
        let prefix = &name[MIN_VAR_FILE_NAME_LEN - 1..];
        let full_path = self
            .path
            .join(String::new() + prefix + &"-" + guid_bytes)
            .join("raw_var");
        let efi_variable = self.parse_payload(&full_path)?;
        if *efi_variable.name != *prefix {
            return Err::<EfiVariable, Box<dyn Error>>(
                "Corrupt variable. Reported name does not match name".into(),
            );
        }
        if efi_variable.guid != guid {
            return Err::<EfiVariable, Box<dyn Error>>(
                "Corrupt variable. Reported guid does not match guid".into(),
            );
        }
        return Ok(efi_variable);
    }

    fn parse_payload(&self, var_path: &Path) -> Result<EfiVariable, Box<dyn Error>> {
        let handle = fs::File::open(var_path)?;
        let mut buffer: Box<dyn EfiNVariableBuffer> = match self.platform_size {
            64 => {
                Ok(Box::new(Efi64VariableBuffer::try_from(handle)?) as Box<dyn EfiNVariableBuffer>)
            }
            32 => {
                Ok(Box::new(Efi32VariableBuffer::try_from(handle)?) as Box<dyn EfiNVariableBuffer>)
            }
            _ => Err(format!("Unsupported platform size: {}", self.platform_size)),
        }?;

        let name: Box<str> = String::from_utf16(
            &(0..EFI_VAR_NAME_LEN)
                .filter_map(|i| {
                    let utf16_char =
                        u16::from_ne_bytes([buffer.name()[2 * i], buffer.name()[2 * i + 1]]);
                    if utf16_char != 0u16 {
                        return Some(utf16_char);
                    }
                    return None;
                })
                .collect::<Vec<u16>>(),
        )?
        .into();
        let guid = EfiGuid::try_from(buffer.guid() as &[u8])?;
        let data_size: usize = match TryInto::<[u8; 8]>::try_into(buffer.data_size()) {
            Ok(v) => Ok::<usize, Box<dyn Error>>(usize::from_ne_bytes(v)),
            Err(_) => Ok::<usize, Box<dyn Error>>(u32::from_ne_bytes(TryInto::<[u8; 4]>::try_into(
                buffer.data_size(),
            )?) as usize),
        }?;
        if data_size > 1024 {
            return Err::<EfiVariable, Box<dyn Error>>(
                "Corrupt variable. Reported data size exceeds maximum".into(),
            );
        }
        let data: Vec<u8> = buffer.data()[0..data_size].into();
        let status: usize = match TryInto::<[u8; 8]>::try_into(buffer.status()) {
            Ok(v) => Ok::<usize, Box<dyn Error>>(usize::from_ne_bytes(v)),
            Err(_) => Ok::<usize, Box<dyn Error>>(u32::from_ne_bytes(TryInto::<[u8; 4]>::try_into(
                buffer.status(),
            )?) as usize),
        }?;
        if status != 0 {
            return Err::<EfiVariable, Box<dyn Error>>(
                format!("Variable read error. Unexpected status code {}", status).into(),
            );
        }
        let attributes = parse_attributes(u32::from_ne_bytes(TryInto::<[u8; 4]>::try_into(
            buffer.attributes(),
        )?));

        return Ok(EfiVariable {
            name,
            guid,
            data,
            attributes,
        });
    }

    fn set_firmware_platform_size(&mut self, size: usize) -> Result<(), Box<dyn Error>> {
        return match size {
            64 => {
                self.platform_size = 64;
                Ok(())
            }
            32 => {
                self.platform_size = 32;
                Ok(())
            }
            _ => Err(format!("Unsupported platform size: {}", size).into()),
        };
    }

    fn get_firmware_platform_size(path: &str) -> Result<usize, Box<dyn Error>> {
        let result = match fs::read(path) {
            Ok(bytes) => match String::from_utf8(bytes) {
                Ok(chars) => {
                    let ws_index = match chars.find(char::is_whitespace) {
                        Some(index) => index,
                        None => chars.len(),
                    };
                    Ok(usize::from_str_radix(&chars[0..ws_index], 10)?)
                }
                Err(e) => Err(e.into()),
            },
            Err(e) => Err(e.into()),
        };
        return result;
    }
}
