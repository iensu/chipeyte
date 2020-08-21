use crate::ChipeyteError;

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

    pub fn get_data_register_value(&self, register: u8) -> Result<u8, ChipeyteError> {
        match register {
            0x0 => Ok(self.v0),
            0x1 => Ok(self.v1),
            0x2 => Ok(self.v2),
            0x3 => Ok(self.v3),
            0x4 => Ok(self.v4),
            0x5 => Ok(self.v5),
            0x6 => Ok(self.v6),
            0x7 => Ok(self.v7),
            0x8 => Ok(self.v8),
            0x9 => Ok(self.v9),
            0xa => Ok(self.va),
            0xb => Ok(self.vb),
            0xc => Ok(self.vc),
            0xd => Ok(self.vd),
            0xe => Ok(self.ve),
            0xf => Ok(self.vf),
            _ => Err(ChipeyteError::BadDataRegister(register)),
        }
    }

    pub fn set_data_register_value(
        &mut self,
        register: u8,
        value: u8,
    ) -> Result<(), ChipeyteError> {
        match register {
            0x0 => {
                self.v0 = value;
                Ok(())
            }
            0x1 => {
                self.v1 = value;
                Ok(())
            }
            0x2 => {
                self.v2 = value;
                Ok(())
            }
            0x3 => {
                self.v3 = value;
                Ok(())
            }
            0x4 => {
                self.v4 = value;
                Ok(())
            }
            0x5 => {
                self.v5 = value;
                Ok(())
            }
            0x6 => {
                self.v6 = value;
                Ok(())
            }
            0x7 => {
                self.v7 = value;
                Ok(())
            }
            0x8 => {
                self.v8 = value;
                Ok(())
            }
            0x9 => {
                self.v9 = value;
                Ok(())
            }
            0xa => {
                self.va = value;
                Ok(())
            }
            0xb => {
                self.vb = value;
                Ok(())
            }
            0xc => {
                self.vc = value;
                Ok(())
            }
            0xd => {
                self.vd = value;
                Ok(())
            }
            0xe => {
                self.ve = value;
                Ok(())
            }
            0xf => {
                self.vf = value;
                Ok(())
            }
            _ => Err(ChipeyteError::BadDataRegister(register)),
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
