use super::efi_guid::EfiGuid;
use super::efi_variable_attribute::EfiVariableAttribute;
use std::collections::HashSet;

pub struct EfiVariable<'a> {
    pub attributes: HashSet<&'a EfiVariableAttribute>,
    pub guid: EfiGuid,
    pub name: Box<str>,
    pub data: Vec<u8>,
}
