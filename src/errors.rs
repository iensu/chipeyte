use crate::Ops;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ChipeyteError {
    OpNotImplemented(Ops),
    BadNumericRegister(u8),
    OpFailed(Ops, String),
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

            ChipeyteError::BadNumericRegister(register) => {
                write!(f, "Unknown numeric register {:x?}", register)
            }
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
