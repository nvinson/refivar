use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum EfiGuidErrorKind {
    BadFormat,
    SliceLengthTooLong,
    SliceLengthTooShort,
    VecLengthTooLong,
    VecLengthTooShort,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct EfiGuidError(EfiGuidErrorKind);

impl Error for EfiGuidError {}

impl fmt::Display for EfiGuidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            EfiGuidErrorKind::BadFormat => write!(
                f,
                "bad format. Correct format is xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"
            ),
            EfiGuidErrorKind::SliceLengthTooLong => {
                write!(f, "source slice too long. Slice must have a size of 16")
            }
            EfiGuidErrorKind::SliceLengthTooShort => {
                write!(f, "source slice too short. Slice must have a size of 16")
            }
            EfiGuidErrorKind::VecLengthTooLong => {
                write!(f, "source vector too long. Vector must have a size of 16")
            }
            EfiGuidErrorKind::VecLengthTooShort => {
                write!(f, "source vector too short. Vector must have a size of 16")
            }
        }
    }
}

pub const BAD_FORMAT: EfiGuidError = EfiGuidError(EfiGuidErrorKind::BadFormat);

pub const SRC_SLICE_LENGTH_TOO_LONG: EfiGuidError =
    EfiGuidError(EfiGuidErrorKind::SliceLengthTooLong);

pub const SRC_SLICE_LENGTH_TOO_SHORT: EfiGuidError =
    EfiGuidError(EfiGuidErrorKind::SliceLengthTooShort);

pub const SRC_VEC_LENGTH_TOO_LONG: EfiGuidError = EfiGuidError(EfiGuidErrorKind::VecLengthTooLong);

pub const SRC_VEC_LENGTH_TOO_SHORT: EfiGuidError =
    EfiGuidError(EfiGuidErrorKind::VecLengthTooShort);
