use crate::types::EfiVariableAttribute;
use std::collections::HashSet;

pub static NON_VOLATILE: EfiVariableAttribute = EfiVariableAttribute::init("Non-Volatile", 0x1);
pub static BOOTSERVICE_ACCESS: EfiVariableAttribute =
    EfiVariableAttribute::init("Boot Service Access", 0x2);
pub static RUNTIME_ACCESS: EfiVariableAttribute =
    EfiVariableAttribute::init("Runtime Service Access", 0x4);
pub static HARDWARE_ERROR_RECORD: EfiVariableAttribute =
    EfiVariableAttribute::init("Hardware Error Record", 0x8);
pub static AUTHENTICATED_WRITE_ACCESS: EfiVariableAttribute =
    EfiVariableAttribute::init("Authenticated Write Access", 0x10);
pub static TIME_BASED_AUTHENTICATED_WRITE_ACCESS: EfiVariableAttribute =
    EfiVariableAttribute::init("Time-Based Authenticated Write Access", 0x20);
pub static APPEND_WRITE: EfiVariableAttribute = EfiVariableAttribute::init("Append Write", 0x40);
pub static ENHANCED_AUTHENTICATED_ACCESS: EfiVariableAttribute =
    EfiVariableAttribute::init("Enhanced Authenticated Access", 0x80);

pub static EFI_VARIABLE_ATTRIBUTES: &'static [&EfiVariableAttribute] = &[
    &NON_VOLATILE,
    &BOOTSERVICE_ACCESS,
    &RUNTIME_ACCESS,
    &HARDWARE_ERROR_RECORD,
    &AUTHENTICATED_WRITE_ACCESS,
    &TIME_BASED_AUTHENTICATED_WRITE_ACCESS,
    &APPEND_WRITE,
    &ENHANCED_AUTHENTICATED_ACCESS,
];

pub fn parse_attributes<'a>(value: u32) -> HashSet<&'a EfiVariableAttribute> {
    let mut set: HashSet<&'a EfiVariableAttribute> = HashSet::new();
    for attr in EFI_VARIABLE_ATTRIBUTES.iter() {
        if u32::from(*attr) & value != 0 {
            set.insert(*attr);
        }
    }
    return set;
}
