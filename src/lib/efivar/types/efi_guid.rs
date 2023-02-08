use std::fmt;

pub struct EfiGuid {
    a: u32,
    b: u16,
    c: u16,
    d: [u8; 8],
}

impl fmt::Display for EfiGuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}", self.a, self.b, self.c,
                        &self.d[0..2].iter().fold(0u16, |sum, b| (sum << 8) | *b as u16),
                        &self.d[2..].iter().fold(0u64, |sum, b| (sum << 8) | *b as u64));
    }
}

impl EfiGuid {
        pub fn from_zero() -> Self {
            return Self { a: 0, b: 0, c: 0, d: [0; 8] };
        }
}
