use crate::ChipeyteError;

#[derive(Debug)]
pub enum NumericRegister {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

impl std::convert::TryFrom<u8> for NumericRegister {
    type Error = ChipeyteError;

    fn try_from(register: u8) -> Result<Self, ChipeyteError> {
        match register {
            0x0 => Ok(NumericRegister::V0),
            0x1 => Ok(NumericRegister::V1),
            0x2 => Ok(NumericRegister::V2),
            0x3 => Ok(NumericRegister::V3),
            0x4 => Ok(NumericRegister::V4),
            0x5 => Ok(NumericRegister::V5),
            0x6 => Ok(NumericRegister::V6),
            0x7 => Ok(NumericRegister::V7),
            0x8 => Ok(NumericRegister::V8),
            0x9 => Ok(NumericRegister::V9),
            0xA => Ok(NumericRegister::VA),
            0xB => Ok(NumericRegister::VB),
            0xC => Ok(NumericRegister::VC),
            0xD => Ok(NumericRegister::VD),
            0xE => Ok(NumericRegister::VE),
            0xF => Ok(NumericRegister::VF),
            _ => Err(ChipeyteError::BadNumericRegister(register)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Registers {
    pub i: u16,  // Stores memory addresses, only lowest 12 bits used.
    pub pc: u16, // program counter
    pub sp: u8,  // Stack pointer
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8, // Not used by any program, used as flag by instructions.
    pub dt: u8, // Delay Timer
    pub st: u8, // Sound Timer
}

impl Registers {
    pub fn new(initial_pc: u16) -> Registers {
        Registers {
            pc: initial_pc,
            ..Default::default()
        }
    }

    pub fn get_numeric_register(&self, register: NumericRegister) -> u8 {
        match register {
            NumericRegister::V0 => self.v0,
            NumericRegister::V1 => self.v1,
            NumericRegister::V2 => self.v2,
            NumericRegister::V3 => self.v3,
            NumericRegister::V4 => self.v4,
            NumericRegister::V5 => self.v5,
            NumericRegister::V6 => self.v6,
            NumericRegister::V7 => self.v7,
            NumericRegister::V8 => self.v8,
            NumericRegister::V9 => self.v9,
            NumericRegister::VA => self.va,
            NumericRegister::VB => self.vb,
            NumericRegister::VC => self.vc,
            NumericRegister::VD => self.vd,
            NumericRegister::VE => self.ve,
            NumericRegister::VF => self.vf,
        }
    }

    pub fn set_numeric_register(&mut self, register: NumericRegister, value: u8) {
        match register {
            NumericRegister::V0 => {
                self.v0 = value;
            }
            NumericRegister::V1 => {
                self.v1 = value;
            }
            NumericRegister::V2 => {
                self.v2 = value;
            }
            NumericRegister::V3 => {
                self.v3 = value;
            }
            NumericRegister::V4 => {
                self.v4 = value;
            }
            NumericRegister::V5 => {
                self.v5 = value;
            }
            NumericRegister::V6 => {
                self.v6 = value;
            }
            NumericRegister::V7 => {
                self.v7 = value;
            }
            NumericRegister::V8 => {
                self.v8 = value;
            }
            NumericRegister::V9 => {
                self.v9 = value;
            }
            NumericRegister::VA => {
                self.va = value;
            }
            NumericRegister::VB => {
                self.vb = value;
            }
            NumericRegister::VC => {
                self.vc = value;
            }
            NumericRegister::VD => {
                self.vd = value;
            }
            NumericRegister::VE => {
                self.ve = value;
            }
            NumericRegister::VF => {
                self.vf = value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_registers_except_pc_default_to_zero() {
        assert_eq!(
            Registers::new(666),
            Registers {
                i: 0,
                pc: 666,
                sp: 0,
                v0: 0,
                v1: 0,
                v2: 0,
                v3: 0,
                v4: 0,
                v5: 0,
                v6: 0,
                v7: 0,
                v8: 0,
                v9: 0,
                va: 0,
                vb: 0,
                vc: 0,
                vd: 0,
                ve: 0,
                vf: 0,
                dt: 0,
                st: 0,
            }
        )
    }
}
