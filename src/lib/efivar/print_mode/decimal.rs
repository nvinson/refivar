use crate::types::EfiVariable;
use std::fmt;

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
