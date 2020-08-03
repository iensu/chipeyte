use crate::cpu::instruction_decoder::Ops;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CPUError {
    OpNotImplemented(Ops),
    BadNumericRegister(u8),
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            CPUError::OpNotImplemented(operation) => {
                write!(f, "Operation {:?} not yet implemented!", operation)
            }

            CPUError::BadNumericRegister(register) => write!(f, "Unknown register {}", register),
        }
    }
}

impl error::Error for CPUError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CPUError::OpNotImplemented(_) => None,
            CPUError::BadNumericRegister(_) => None,
        }
    }
}
