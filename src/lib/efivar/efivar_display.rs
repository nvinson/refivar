use crate::types::{EfiVariable, EfiVariableAttribute};
use std::fmt;

pub struct Verbose<'a>(pub &'a EfiVariable<'a>);
pub struct Decimal<'a>(pub &'a EfiVariable<'a>);

impl fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.data.iter().enumerate() {
            if (i > 0) && (i % 16 == 0) {
                f.write_str("  ")?;
            }
            if i % 16 < 8 {
                f.write_str(&format!("{} ", b))?;
            } else {
                f.write_str(&format!(" {}", b))?;
            }
        }
        return Ok(());
    }
}

impl fmt::Display for Verbose<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("GUID: {}\n", self.0.guid))?;
        f.write_str(&format!("Name: \"{}\"\n", self.0.name))?;
        f.write_str("Attributes:\n")?;
        let mut sorted_attrs: Vec<&EfiVariableAttribute> =
            self.0.attributes.clone().into_iter().collect();
        sorted_attrs.sort_unstable_by(|a1, a2| <u32>::from(*a1).cmp(&<u32>::from(*a2)));
        for a in sorted_attrs.iter() {
            f.write_str(&format!("\t{}\n", String::from(*a)))?;
        }

        f.write_str("Value:\n")?;
        for i in (0..self.0.data.len()).step_by(16) {
            f.write_str(&format!("{:08x}  ", i))?;

            let mut decode: [u8; 16] = [0; 16];
            let mut line_iter = self.0.data.iter().skip(i);
            for j in i..i + 16 {
                match line_iter.next() {
                    Some(c) => {
                        if (0x20 < *c) && (*c < 0x7e) {
                            decode[j - i] = u32::from(*c) as u8;
                        } else {
                            decode[j - i] = u32::from('.') as u8;
                        }
                        if j < i + 8 {
                            f.write_str(&format!("{:02x} ", u8::from(*c)))?;
                        } else {
                            f.write_str(&format!(" {:02x}", u8::from(*c)))?;
                        }
                    }
                    None => {
                        decode[j - i] = u32::from(' ') as u8;
                        f.write_str("   ")?;
                    }
                };
            }
            f.write_str(&format!("  |{}|\n", std::str::from_utf8(&decode).unwrap()))?;
        }
        f.write_str(&format!("{:08x}\n", self.0.data.len()))?;
        return Ok(());
    }
}
