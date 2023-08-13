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
    value: u32,
}

impl fmt::Display for EfiVariableAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "EfiVariableAttribute {{name: \"{}\", value: {:#08x}}}",
            self.name, self.value
        );
    }
}

EfiVariableAttribute_field_value_maps!(u128, u64, u32, i128, i64, i32);

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
    pub(crate) const fn new(name: &'static str, value: u32) -> Self {
        return Self { name, value };
    }

    pub fn name(&self) -> &'static str {
        return self.name;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_uint128() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u128 << i, u128::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u128 << i, u128::from(&attr));
        })
    }

    #[test]
    fn to_int128() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i128 << i, i128::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i128 << i, i128::from(&attr));
        })
    }

    #[test]
    fn to_uint64() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u64 << i, u64::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u64 << i, u64::from(&attr));
        })
    }

    #[test]
    fn to_int64() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i64 << i, i64::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i64 << i, i64::from(&attr));
        })
    }

    #[test]
    fn to_uint32() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u32 << i, u32::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1u32 << i, u32::from(&attr));
        })
    }

    #[test]
    fn to_int32() {
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i32 << i, i32::from(attr));

            let attr = EfiVariableAttribute::new("Test attribute", 1 << i);
            assert_eq!(1i32 << i, i32::from(&attr));
        });
    }

    #[test]
    fn to_str() {
        let name = "Test attribute";
        (0..31).for_each(|i| {
            let attr = EfiVariableAttribute::new(name, 1 << i);
            assert_eq!(name, String::from(&attr));

            let attr = EfiVariableAttribute::new(name, 1 << i);
            assert_eq!(name, String::from(&attr));
        });
    }

    #[test]
    fn display() {
        let name = "Test attribute";
        (0..31).for_each(|i| {
            let value = 1 << i;

            let attr = EfiVariableAttribute::new(name, value);
            assert_eq!(
                format!(
                    "EfiVariableAttribute {{name: \"{}\", value: {:#08x}}}",
                    name, value
                ),
                format!("{}", attr)
            );
        });
    }
}
