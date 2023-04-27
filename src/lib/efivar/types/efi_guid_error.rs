use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EfiGuidError {
    BadFormat,
    SliceLengthTooLong,
    SliceLengthTooShort,
    VecLengthTooLong,
    VecLengthTooShort,
}

impl fmt::Display for EfiGuidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BadFormat => write!(
                f,
                "bad format. Correct format is xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            ),
            Self::SliceLengthTooLong => {
                write!(f, "source slice too long. Slice must have a size of 16")
            }
            Self::SliceLengthTooShort => {
                write!(f, "source slice too short. Slice must have a size of 16")
            }
            Self::VecLengthTooLong => {
                write!(f, "source vector too long. Vector must have a size of 16")
            }
            Self::VecLengthTooShort => {
                write!(f, "source vector too short. Vector must have a size of 16")
            }
        }
    }
}

impl Error for EfiGuidError {};