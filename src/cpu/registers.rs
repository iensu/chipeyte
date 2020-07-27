#[derive(Debug, PartialEq, Eq, Default)]
pub struct Registers {
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
}

impl Registers {
    pub fn new(initial_pc: u16) -> Registers {
        Registers {
            pc: initial_pc,
            ..Default::default()
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
            }
        )
    }
}
