use crate::types::efi_guid::EfiGuid;
use crate::types::efi_variable_attribute::EfiVariableAttribute;
use std::collections::HashSet;

pub struct EfiVariable<'a> {
    pub attributes: HashSet<&'a EfiVariableAttribute>,
    pub guid: EfiGuid,
    pub name: Box<str>,
    pub data: Vec<u8>,
}
