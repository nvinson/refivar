use std::fmt;

macro_rules! EfiVariableAttribute_field_value_maps {
    ($type:ident) => {
        impl From<EfiVariableAttribute> for $type {
            fn from(attr: EfiVariableAttribute) -> $type {
                return attr.value as $type;
            }
        }

        impl From<&EfiVariableAttribute> for $type {
            fn from(attr: &EfiVariableAttribute) -> $type {
                return (*attr).value as $type;
            }
        }
    };
    ($type:ident, $($types:ident),+) => {
        EfiVariableAttribute_field_value_maps!($type);
        EfiVariableAttribute_field_value_maps!($($types),+);
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct EfiVariableAttribute {
    name: &'static str,
    value: u64,
}

impl fmt::Display for EfiVariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "EfiVariableAttribute {{name: \"{}\", value: {:#016x}}}",
            self.name, self.value
        );
    }
}

EfiVariableAttribute_field_value_maps!(u128, u64, u32, u16, i128, i64, i32);

impl From<EfiVariableAttribute> for String {
    fn from(attr: EfiVariableAttribute) -> String {
        return String::from(attr.name);
    }
}

impl From<&EfiVariableAttribute> for String {
    fn from(attr: &EfiVariableAttribute) -> String {
        return String::from((*attr).name);
    }
}

impl EfiVariableAttribute {
    pub(crate) const fn init(name: &'static str, value: u64) -> Self {
        return Self { name, value };
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }
}
