use std::fmt;

macro_rules! EfivarAttribute_field_value_maps {
    ($type:ident) => {
        impl From<EfivarAttribute> for $type {
            fn from(attr: EfivarAttribute) -> $type {
                return attr.value as $type;
            }
        }

        impl From<&EfivarAttribute> for $type {
            fn from(attr: &EfivarAttribute) -> $type {
                return (*attr).value as $type;
            }
        }
    };
    ($type:ident, $($types:ident),+) => {
        EfivarAttribute_field_value_maps!($type);
        EfivarAttribute_field_value_maps!($($types),+);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EfivarAttribute {
    name: &'static str,
    value: u64,
}

impl fmt::Display for EfivarAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "EfivarAttribute {{name: \"{}\", value: {:#016x}}}", self.name, self.value);
    }
}

EfivarAttribute_field_value_maps!(u128, u64, u32, u16, i128, i64, i32);

impl From<EfivarAttribute> for String {
    fn from(attr: EfivarAttribute) -> String {
        return String::from(attr.name);
    }
}

impl From<&EfivarAttribute> for String {
    fn from(attr: &EfivarAttribute) -> String {
        return String::from((*attr).name);
    }
}

impl EfivarAttribute {
    pub(crate) const fn init(name: &'static str, value: u64) -> Self {
        return Self {name, value};
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }
}
