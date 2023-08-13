use crate::efi_variable_attributes::parse_attributes;
use crate::efivarfs;
use crate::types::{EfiGuid, EfiVariable};
use crate::MIN_VAR_FILE_NAME_LEN;
use std::boxed::Box;
use std::convert::TryFrom;
use std::error::Error;
use std::io::{self, ErrorKind, IoSliceMut, Read};
use std::path::{Path, PathBuf};

#[cfg(not(test))]
use std::fs::File;
#[cfg(test)]
use tests::File;

const EFI_VAR_NAME_LEN: usize = 512;
const EFIVARS_FW_PLATFORM_SZ_PATH: &'static str = "/sys/firmware/efi/fw_platform_size";
const EFIVARS_PATH: &'static str = "/sys/firmware/efi/vars";

pub struct EfiVariables {
    path: PathBuf,
    platform_size: usize,
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

#[derive(Debug, PartialEq)]
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

impl<const SIZE: usize> TryFrom<&mut dyn Read> for EfiVariableBuffer<SIZE> {
    type Error = Box<dyn Error>;

    fn try_from(handle: &mut dyn Read) -> Result<Self, Self::Error> {
        let mut buffer = Self::new();
        let total_buffer_size = match SIZE {
            8 => Ok(2084), // cannot use size_of with EfiVariableBuffer<SIZE>
            4 => Ok(2076), // so using manually computed sizes.
            _ => Err(format!("Unsupported size: {}", SIZE)),
        }?;
        let mut total_bytes_read = 0;
        while total_bytes_read < total_buffer_size {
            let mut io_vectors: Vec<IoSliceMut> = Vec::new();
            let mut offset = total_bytes_read;
            [
                &mut buffer.name as &mut [u8],
                &mut buffer.guid as &mut [u8],
                &mut buffer.data_size as &mut [u8],
                &mut buffer.data as &mut [u8],
                &mut buffer.status as &mut [u8],
                &mut buffer.attributes as &mut [u8],
            ]
            .into_iter()
            .for_each(|b| {
                if offset < b.len() {
                    if offset == 0 {
                        io_vectors.push(IoSliceMut::new(b));
                    } else {
                        io_vectors.push(IoSliceMut::new(&mut b[offset..]));
                        offset = 0;
                    }
                } else {
                    offset -= b.len();
                }
            });
            match handle.read_vectored(io_vectors.as_mut_slice()) {
                Ok(bytes_read) => {
                    eprintln!("read bytes {}", bytes_read);
                    if bytes_read == 0 {
                        return Err(format!(
                            "Corrupt variable. Read {} byte(s) but expected to read {}.",
                            total_bytes_read, total_buffer_size
                        )
                        .into());
                    }
                    total_bytes_read += bytes_read;
                }
                Err(e) => {
                    if e.kind() == ErrorKind::Interrupted {
                        continue;
                    }
                    return Err(Box::new(e));
                }
            };
        }
        let mut buf: [u8; 1] = [0];
        loop {
            match handle.read(&mut buf) {
                Ok(_) => {
                    return Err(format!(
                        "Corrupt variable. Read {} byte(s) but expected to read {}.",
                        total_buffer_size + 1,
                        total_buffer_size
                    )
                    .into());
                }
                Err(e) => {
                    if e.kind() == ErrorKind::Interrupted {
                        continue;
                    }
                    if e.kind() == ErrorKind::UnexpectedEof {
                        return Ok(buffer);
                    }
                    return Err(Box::new(e));
                }
            }
        }
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
    pub fn new() -> Self {
        let mut variables = EfiVariables {
            path: EFIVARS_PATH.into(),
            platform_size: 0,
        };
        variables
            .set_firmware_platform_size(
                EfiVariables::get_firmware_platform_size(&PathBuf::from(
                    EFIVARS_FW_PLATFORM_SZ_PATH,
                ))
                .unwrap(),
            )
            .unwrap();
        return variables;
    }

    pub fn set_path(&mut self, path: PathBuf) -> &EfiVariables {
        self.path = path;
        return self;
    }

    pub fn list(&self) -> io::Result<efivarfs::EfiVariablesNameIter> {
        let mut e: efivarfs::EfiVariables = efivarfs::EfiVariables::new();
        e.set_path(self.path.clone());
        return e.list();
    }

    pub fn get_variable(&self, name: &str) -> Result<EfiVariable, Box<dyn Error>> {
        if name.len() < MIN_VAR_FILE_NAME_LEN
            || name.bytes().nth(MIN_VAR_FILE_NAME_LEN - 2).unwrap() != b'-'
        {
            return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
        }
        let prefix = &name[MIN_VAR_FILE_NAME_LEN - 1..];
        let guid = match EfiGuid::try_from(&name[0..MIN_VAR_FILE_NAME_LEN - 2]) {
            Ok(g) => Some(g),
            Err(_) => {
                return Err(io::Error::from(io::ErrorKind::InvalidInput).into());
            }
        }
        .unwrap();
        let efi_variable_path = self
            .path
            .join(String::new() + prefix + &"-" + format!("{}", guid).as_str())
            .join("raw_var");
        let mut handle = File::open(efi_variable_path)?;

        let efi_variable = self.parse_payload(&mut handle)?;
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

    fn parse_payload(&self, reader: &mut dyn Read) -> Result<EfiVariable, Box<dyn Error>> {
        let mut buffer: Box<dyn EfiNVariableBuffer> = match self.platform_size {
            64 => {
                Ok(Box::new(Efi64VariableBuffer::try_from(reader)?) as Box<dyn EfiNVariableBuffer>)
            }
            32 => {
                Ok(Box::new(Efi32VariableBuffer::try_from(reader)?) as Box<dyn EfiNVariableBuffer>)
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

    fn get_firmware_platform_size(path: &Path) -> Result<usize, Box<dyn Error>> {
        let mut handle = File::open(path)?;
        let mut chars: String = String::new();

        return match handle.read_to_string(&mut chars) {
            Ok(_) => {
                let ws_index = match chars.find(char::is_whitespace) {
                    Some(index) => index,
                    None => chars.len(),
                };
                Ok(usize::from_str_radix(&chars[0..ws_index], 10)?)
            }
            Err(e) => Err(e.into()),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::min;
    use std::collections::VecDeque;
    use std::io::{Read, Write};
    use std::sync::Mutex;

    static BUFFER_MOCK: Mutex<VecDeque<u8>> = Mutex::new(VecDeque::new());
    static RUNNER_LOCK: Mutex<u8> = Mutex::new(0xff);

    pub struct File<'a> {
        buffer: &'a Mutex<VecDeque<u8>>,
    }

    impl File<'_> {
        pub fn open<P: AsRef<Path>>(_path: P) -> std::io::Result<Self> {
            return Ok(Self {
                buffer: &BUFFER_MOCK,
            });
        }
    }

    impl Read for File<'_> {
        fn read(&mut self, dst: &mut [u8]) -> std::io::Result<usize> {
            return self.buffer.lock().unwrap().read(dst);
        }

        fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> std::io::Result<usize> {
            let mut inner_buffer = self.buffer.lock().unwrap();
            let mut total_bytes_read: usize = 0;
            bufs.iter_mut().filter(|b| !b.is_empty()).for_each(|b| {
                let bytes_read = min(b.len(), inner_buffer.len());
                b[..bytes_read].copy_from_slice(
                    &inner_buffer
                        .drain(..bytes_read)
                        .collect::<Vec<u8>>()
                        .as_slice(),
                );
                total_bytes_read += bytes_read;
            });

            return Ok(total_bytes_read);
        }
    }

    #[test]
    fn get_firmware_platform_size() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write("32\n".as_bytes()).unwrap();
        }
        assert_eq!(
            EfiVariables::get_firmware_platform_size(&PathBuf::from("/tmp/unit_test_refivar"))
                .unwrap(),
            32
        );

        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write("64\n".as_bytes()).unwrap();
        }
        assert_eq!(
            EfiVariables::get_firmware_platform_size(&PathBuf::from("/tmp/unit_test_refivar"))
                .unwrap(),
            64
        );

        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write("1\n".as_bytes()).unwrap();
        }
        assert_eq!(
            EfiVariables::get_firmware_platform_size(&PathBuf::from("/tmp/unit_test_refivar"))
                .unwrap(),
            1
        );
    }

    #[test]
    fn set_firmware_platform_size() {
        let mut efi_variables = EfiVariables {
            path: "".into(),
            platform_size: 0,
        };

        efi_variables.set_firmware_platform_size(64).unwrap();
        assert_eq!(efi_variables.platform_size, 64);

        efi_variables.set_firmware_platform_size(32).unwrap();
        assert_eq!(efi_variables.platform_size, 32);

        let result = efi_variables.set_firmware_platform_size(36).err();
        assert_eq!(
            (*(result.unwrap())).to_string(),
            "Unsupported platform size: 36"
        );
    }

    #[test]
    fn efi_variable_buffer_32_read_empty() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
        }
        let var = Efi32VariableBuffer::try_from(
            BUFFER_MOCK.lock().as_deref_mut().unwrap() as &mut dyn Read
        );

        assert_eq!(
            (*var.err().unwrap()).to_string(),
            "Corrupt variable. Read 0 byte(s) but expected to read 2076."
        );
    }

    #[test]
    fn efi_variable_buffer_32_read_short() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write("1".as_bytes()).unwrap();
        }
        let var = Efi32VariableBuffer::try_from(
            BUFFER_MOCK.lock().as_deref_mut().unwrap() as &mut dyn Read
        );

        assert_eq!(
            (*var.err().unwrap()).to_string(),
            "Corrupt variable. Read 1 byte(s) but expected to read 2076."
        );
    }

    #[test]
    fn efi_variable_buffer_32_read_too_long() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write(&[0xff; 2077]).unwrap();
        }
        let mut mock_stream = File {
            buffer: &BUFFER_MOCK,
        };
        let var = Efi32VariableBuffer::try_from(&mut mock_stream as &mut dyn Read);

        assert_eq!(
            (*var.err().unwrap()).to_string(),
            "Corrupt variable. Read 2077 byte(s) but expected to read 2076."
        );
    }

    #[test]
    fn efi_variable_buffer_64_read_short() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write("1".as_bytes()).unwrap();
        }
        let var = Efi64VariableBuffer::try_from(
            BUFFER_MOCK.lock().as_deref_mut().unwrap() as &mut dyn Read
        );

        assert_eq!(
            (*var.err().unwrap()).to_string(),
            "Corrupt variable. Read 1 byte(s) but expected to read 2084."
        );
    }

    #[test]
    fn efi_variable_buffer_64_read_too_long() {
        let _lock = RUNNER_LOCK.lock();
        {
            let mut mock_buffer = BUFFER_MOCK.lock().unwrap();
            mock_buffer.clear();
            mock_buffer.write(&[0xff; 2085]).unwrap();
        }
        let mut mock_stream = File {
            buffer: &BUFFER_MOCK,
        };
        let var = Efi64VariableBuffer::try_from(&mut mock_stream as &mut dyn Read);

        assert_eq!(
            (*var.err().unwrap()).to_string(),
            "Corrupt variable. Read 2085 byte(s) but expected to read 2084."
        );
    }
}
