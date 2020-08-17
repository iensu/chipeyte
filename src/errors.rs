use crate::Ops;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ChipeyteError {
    OpNotImplemented(Ops),
    BadDataRegister(u8),
    OpFailed(Ops, String),
    UnsupportedSprite(u8),
    UnknownKey(u8),
}

impl fmt::Display for ChipeyteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            ChipeyteError::OpNotImplemented(op) => {
                write!(f, "Operation {:?} not yet implemented!", op)
            }

            ChipeyteError::OpFailed(op, msg) => {
                write!(f, "Operation {:?} failed with message: {}", op, msg)
            }

            ChipeyteError::BadDataRegister(register) => {
                write!(f, "Unknown data register {:x?}", register)
            }

            ChipeyteError::UnsupportedSprite(digit) => write!(f, "Unsupported sprite {:x?}", digit),

            ChipeyteError::UnknownKey(key) => write!(f, "Unknown key: {:x?}", key),
        }
    }
}

impl error::Error for ChipeyteError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}
