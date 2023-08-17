use crate::types::EfiVariable;
use std::fmt;

pub struct Decimal<'a>(pub &'a EfiVariable<'a>);

impl fmt::Display for Decimal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.0.data.iter().enumerate() {
            if (i > 0) && (i != self.0.data.len() - 1) && (i % 16 == 0) {
                f.write_str("  ")?;
            }
            if i % 16 < 8 {
                f.write_str(&format!(
                    "{}{}",
                    b,
                    if i != self.0.data.len() - 1 { " " } else { "" }
                ))?;
            } else {
                f.write_str(&format!(" {}", b))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::EfiGuid;
    use crate::types::EfiVariable;
    use std::collections::HashSet;

    #[test]
    fn test_display_no_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [].to_vec(),
        };
        assert_eq!("", format!("{}", Decimal(&var)));
    }

    #[test]
    fn test_display_half_word_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [0, 1, 2, 3].to_vec(),
        };
        assert_eq!("0 1 2 3", format!("{}", Decimal(&var)));
    }

    #[test]
    fn test_display_word_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07].to_vec(),
        };
        assert_eq!("0 1 2 3 4 5 6 7", format!("{}", Decimal(&var)));
    }

    #[test]
    fn test_display_double_word_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f,
            ]
            .to_vec(),
        };
        assert_eq!(
            "0 1 2 3 4 5 6 7  8 9 10 11 12 13 14 15",
            format!("{}", Decimal(&var))
        );
    }

    #[test]
    fn test_display_triple_word_data() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            ]
            .to_vec(),
        };
        assert_eq!(
            "".to_owned() + "0 1 2 3 4 5 6 7  8 9 10 11 12 13 14 15  " + "16 17 18 19 20 21 22 23",
            format!("{}", Decimal(&var))
        );
    }

    #[test]
    fn test_quad_word_display() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f,
            ]
            .to_vec(),
        };
        assert_eq!(
            "".to_owned()
                + "0 1 2 3 4 5 6 7  8 9 10 11 12 13 14 15  "
                + "16 17 18 19 20 21 22 23  24 25 26 27 28 29 30 31",
            format!("{}", Decimal(&var))
        );
    }

    #[test]
    fn test_oct_word_display() {
        let var = EfiVariable {
            attributes: HashSet::new(),
            guid: EfiGuid::try_from("12345678-1234-1234-1234-12345678abcd").unwrap(),
            name: "Unit Test Variable".into(),
            data: [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
                0x1c, 0x1d, 0x1e, 0x1f, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9,
                0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7,
                0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff,
            ]
            .to_vec(),
        };
        assert_eq!(
            "".to_owned()
                + "0 1 2 3 4 5 6 7  8 9 10 11 12 13 14 15  "
                + "16 17 18 19 20 21 22 23  24 25 26 27 28 29 30 31  "
                + "224 225 226 227 228 229 230 231  232 233 234 235 236 237 238 239  "
                + "240 241 242 243 244 245 246 247  248 249 250 251 252 253 254 255",
            format!("{}", Decimal(&var))
        );
    }
}
