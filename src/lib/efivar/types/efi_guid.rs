use crate::types::EfiGuidError;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, Copy)]
pub struct EfiGuid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

impl Ord for EfiGuid {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.a.cmp(&other.a) != Ordering::Equal {
            return self.a.cmp(&other.a);
        }
        if self.b.cmp(&other.b) != Ordering::Equal {
            return self.b.cmp(&other.b);
        }
        if self.c.cmp(&other.c) != Ordering::Equal {
            return self.c.cmp(&other.c);
        }
        for i in 0..7 {
            if self.d[7 - i].cmp(&other.d[7 - i]) != Ordering::Equal {
                return self.d[7 - i].cmp(&other.d[7 - i]);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for EfiGuid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EfiGuid {
    fn eq(&self, other: &Self) -> bool {
        return self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d;
    }
}

impl fmt::Display for EfiGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            self.a,
            self.b,
            self.c,
            &self.d[0..2]
                .iter()
                .fold(0u16, |sum, b| (sum << 8) | *b as u16),
            &self.d[2..]
                .iter()
                .fold(0u64, |sum, b| (sum << 8) | *b as u64)
        );
    }
}

macro_rules! from_byte_array {
    ($type:ty) => {
        impl From<$type> for EfiGuid {
            fn from(value: $type) -> Self {
                return Self {
                    a: (u32::from(value[3] as u8) << 24)
                        | (u32::from(value[2] as u8) << 16)
                        | (u32::from(value[1] as u8) << 8)
                        | (u32::from(value[0] as u8)),
                    b: (u16::from(value[5] as u8) << 8) | (u16::from(value[4] as u8)),
                    c: (u16::from(value[7] as u8) << 8) | (u16::from(value[6] as u8)),
                    d: [
                        (value[8] as u8),
                        (value[9] as u8),
                        (value[10] as u8),
                        (value[11] as u8),
                        (value[12] as u8),
                        (value[13] as u8),
                        (value[14] as u8),
                        (value[15] as u8),
                    ],
                };
            }
        }
    };
}

macro_rules! from_byte_arrays {
    ($type:ty) => {
        from_byte_array!(&[$type; 16]);
        from_byte_array!(&mut [$type; 16]);
    };
}

macro_rules! try_from_byte_slice {
    ($type:ty) => {
        impl TryFrom<&[$type]> for EfiGuid {
            type Error = EfiGuidError;

            fn try_from(value: &[$type]) -> Result<Self, EfiGuidError> {
                let len = value.len();

                return match <&[$type; 16]>::try_from(value) {
                    Ok(o) => Ok(Self::from(o)),
                    Err(_) => {
                        if len < 16 {
                            Err(EfiGuidError::SliceLengthTooShort)
                        } else {
                            Err(EfiGuidError::SliceLengthTooLong)
                        }
                    }
                };
            }
        }
    };
}

macro_rules! try_from_byte_vec {
    ($type:ty) => {
        impl TryFrom<Vec<$type>> for EfiGuid {
            type Error = EfiGuidError;

            fn try_from(value: Vec<$type>) -> Result<Self, EfiGuidError> {
                let vec_len = value.len();
                match <[$type; 16]>::try_from(value) {
                    Ok(a) => Ok(EfiGuid::from(&a)),
                    Err(_o) => {
                        if vec_len < 16 {
                            Err(EfiGuidError::VecLengthTooShort)
                        } else {
                            Err(EfiGuidError::VecLengthTooLong)
                        }
                    }
                }
            }
        }
    };
}

macro_rules! from_bytes {
    ($type:ty) => {
        from_byte_arrays!($type);
        try_from_byte_slice!($type);
        try_from_byte_vec!($type);
    };
    ($type:ty, $($types:ty),+) => {
        from_bytes!($type);
        from_bytes!($($types),+);
    }
}

from_bytes!(i8, u8);

impl FromStr for EfiGuid {
    type Err = EfiGuidError;

    fn from_str(value: &str) -> Result<Self, EfiGuidError> {
        let chars: Vec<char> = value.chars().into_iter().collect();
        if chars.len() != 36 {
            return Err(EfiGuidError::BadFormat);
        }
        let digits = chars
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if i == 8 || i == 13 || i == 18 || i == 23 {
                    if c != '-' {
                        return Some(Err(EfiGuidError::BadFormat));
                    }
                    return None;
                }
                return match c.to_digit(16) {
                    Some(d) => Some(Ok(d as u8)),
                    None => Some(Err(EfiGuidError::BadFormat)),
                };
            })
            .collect::<Result<Vec<u8>, _>>()?;

        return Ok(Self {
            a: digits[0..8]
                .iter()
                .fold(0u32, |sum, d| (sum << 4) + u32::from(*d)),
            b: digits[8..12]
                .iter()
                .fold(0u16, |sum, d| (sum << 4) + u16::from(*d)),
            c: digits[12..16]
                .iter()
                .fold(0u16, |sum, d| (sum << 4) + u16::from(*d)),
            d: match <[u8; 8]>::try_from(
                iter::zip(
                    digits[16..].into_iter().step_by(2),
                    digits[16..].into_iter().skip(1).step_by(2),
                )
                .map(|(h, l)| (h << 4) + l)
                .collect::<Vec<u8>>(),
            ) {
                Ok(o) => Ok(o),
                Err(_) => Err(EfiGuidError::BadFormat),
            }
            .unwrap(),
        });
    }
}

impl TryFrom<&str> for EfiGuid {
    type Error = EfiGuidError;

    fn try_from(value: &str) -> Result<Self, EfiGuidError> {
        return value.parse::<Self>();
    }
}
