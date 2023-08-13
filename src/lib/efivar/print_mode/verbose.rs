use crate::types::{EfiVariable, EfiVariableAttribute};
use std::fmt;

pub struct Verbose<'a>(pub &'a EfiVariable<'a>);

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
                        if (0x1f < *c) && (*c < 0x7f) {
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
        f.write_str(&format!("{:08x}", self.0.data.len()))?;
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EfiGuid;
    use crate::types::EfiVariable;
    use indoc::indoc;
    use std::collections::HashSet;

    #[test]
    fn test_display_no_name_no_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "".into(),
            data: [].to_vec(),
        };
        assert_eq!(
            indoc!(
                r#"
            GUID: 12345678-1234-1234-1234-12345678abcd
            Name: ""
            Attributes:
            Value:
            00000000"#
            ),
            format!("{}", Verbose(&var))
        );
    }

    #[test]
    fn test_display_no_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [].to_vec(),
        };
        assert_eq!(
            indoc!(
                r#"
            GUID: 12345678-1234-1234-1234-12345678abcd
            Name: "Unit Test Variable"
            Attributes:
            Value:
            00000000"#
            ),
            format!("{}", Verbose(&var))
        );
    }

    #[test]
    fn test_display_16_byte_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [0; 16].to_vec(),
        };
        assert_eq!(
            indoc!(
                r#"
            GUID: 12345678-1234-1234-1234-12345678abcd
            Name: "Unit Test Variable"
            Attributes:
            Value:
            00000000  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
            00000010"#
            ),
            format!("{}", Verbose(&var))
        );
    }

    #[test]
    fn test_display_17_byte_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [0; 17].to_vec(),
        };
        assert_eq!(
            indoc!(
                r#"
            GUID: 12345678-1234-1234-1234-12345678abcd
            Name: "Unit Test Variable"
            Attributes:
            Value:
            00000000  00 00 00 00 00 00 00 00  00 00 00 00 00 00 00 00  |................|
            00000010  00                                                |.               |
            00000011"#
            ),
            format!("{}", Verbose(&var))
        );
    }

    #[test]
    fn test_display_mixed_byte_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [
                0x00, 0x00, 0x54, 0x65, 0x73, 0x74, 0x20, 0x50,
                0x61, 0x73, 0x73, 0x65, 0x64, 0x21, 0x00, 0x00,
            ]
            .to_vec(),
        };
        assert_eq!(
            indoc!(
                r#"
            GUID: 12345678-1234-1234-1234-12345678abcd
            Name: "Unit Test Variable"
            Attributes:
            Value:
            00000000  00 00 54 65 73 74 20 50  61 73 73 65 64 21 00 00  |..Test Passed!..|
            00000010"#
            ),
            format!("{}", Verbose(&var))
        );
    }
}
