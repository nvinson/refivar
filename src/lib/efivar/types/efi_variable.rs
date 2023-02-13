use std::fmt;

pub use super::efi_guid::EfiGuid;
pub use super::efi_variable_attribute::EfiVariableAttribute;

pub struct EfiVariable<'a> {
    pub attributes: Vec<&'a EfiVariableAttribute>,
    pub guid: EfiGuid,
    pub name: &'a str,
    pub data: Vec<u8>,
}

impl fmt::Display for EfiVariable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("EfiVariable {{ attributes: ["))?;
        for a in self.attributes.iter() {
            f.write_str(&format!("{}", *a)[..])?;
        }
        f.write_str(&format!("], guid: \"{}\", name: \"{}\", data: [", self.guid, self.name)[..])?;
        if self.data.len() > 0 {
            f.write_str(&format!("{:02x}", self.data[0])[..])?;
        }
        if self.data.len() > 1 {
            for b in self.data[1..].iter() {
                f.write_str(&format!(" {:02x}", *b)[..])?;
            };
        }
        f.write_str("] }}")?;

        return Ok(());
    }
}
