use std::fmt;

pub struct EfivarAttribute {
    name: &'static str,
    value: u64,
}

impl fmt::Display for EfivarAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "EfivarAttribute {{name: \"{}\", value: {:#016x}}}", self.name, self.value);
    }
}

impl EfivarAttribute {
    pub(crate) const fn init(name: &'static str, value: u64) -> Self {
        return Self {name, value};
    }
}
