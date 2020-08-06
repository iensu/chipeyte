use crate::{cpu::registers::NumericRegister, types::*, ChipeyteError, Memory, Registers};
use std::convert::TryFrom;

const INSTRUCTION_LENGTH: u16 = 2;
const STACK_ENTRY_LENGTH: u8 = 2;

pub trait Callable {
    fn call(&self, register: &mut Registers, memory: &mut Memory) -> Result<(), ChipeyteError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ops {
    UNKNOWN(u16),
    SYS(Addr),
    CLS,
    RET,
    JP(Addr),
    CALL(Addr),
    SE(V, Byte),
    SEV(V, V),
    SNE(V, Byte),
    LD(V, Byte),
    ADD(V, Byte),
    LDV(V, V),
    OR(V, V),
    AND(V, V),
    XOR(V, V),
    ADDV(V, V),
    SUB(V, V),
    SHR(V),
    SUBN(V, V),
    SHL(V),
    SNEV(V, V),
    LDI(Addr),
    JPV0(Addr),
    RND(V, Byte),
    DRW(V, V, Nibble),
    SKP(V),
    SKNP(V),
    LDVDT(V),
    LDK(V),
    LDDT(V),
    LDST(V),
    ADDI(V),
    LDF(V),
    LDB(V),
    LDIV(V),
    LDVI(V),
}

impl Callable for Ops {
    fn call(&self, registers: &mut Registers, memory: &mut Memory) -> Result<(), ChipeyteError> {
        match &*self {
            Ops::UNKNOWN(op) => Err(ChipeyteError::OpFailed(
                *self,
                format!("Unknown operation: {:04x?}", op),
            )),

            Ops::SYS(_) => Ok(()),

            Ops::CLS => Err(ChipeyteError::OpNotImplemented(*self)),

            Ops::RET => {
                registers.pc = memory.get_u16(registers.sp.into());
                registers.sp -= STACK_ENTRY_LENGTH;
                Ok(())
            }

            Ops::JP(addr) => {
                registers.pc = *addr;
                Ok(())
            }

            Ops::CALL(addr) => {
                registers.sp += STACK_ENTRY_LENGTH;
                memory.set_u16(registers.sp.into(), registers.pc);
                registers.pc = *addr;
                Ok(())
            }

            Ops::SE(v, byte) => {
                let register = NumericRegister::try_from(*v)?;
                let value = registers.get_numeric_register(&register);

                if value == *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SNE(v, byte) => {
                let register = NumericRegister::try_from(*v)?;
                let value = registers.get_numeric_register(&register);

                if value != *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SEV(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx)?;
                let reg_y = NumericRegister::try_from(*vy)?;
                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                if x == y {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LD(v, byte) => {
                let register = NumericRegister::try_from(*v)?;

                registers.set_numeric_register(&register, *byte);
                Ok(())
            }

            Ops::ADD(v, byte) => {
                let register = NumericRegister::try_from(*v)?;

                let value = registers.get_numeric_register(&register);
                let result = (u8::MAX as u16).min(*byte as u16 + value as u16);
                registers.set_numeric_register(&register, result as u8);
                Ok(())
            }

            Ops::LDV(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::OR(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::AND(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::XOR(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::ADDV(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SUB(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SHR(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SUBN(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SHL(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SNEV(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDI(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::JPV0(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::RND(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::DRW(_, _, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SKP(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SKNP(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDVDT(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDK(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDDT(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDST(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::ADDI(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDF(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDB(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDIV(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDVI(_) => Err(ChipeyteError::OpNotImplemented(*self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM_START: u16 = 0x0200;
    const INSTRUCTION_LENGTH: u16 = 2;

    #[test]
    fn op_sys_is_ignored() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::SYS(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers, Registers::new(PROGRAM_START));
        assert_eq!(memory, Memory::new());
    }

    #[test]
    fn op_ret_returns() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.sp, 0x0002);
        assert_eq!(registers.pc, 0x0aaa);

        Ops::RET.call(&mut registers, &mut memory).unwrap();

        assert_eq!(memory.get_u16(0x0002), 0x0200);
        assert_eq!(registers.sp, 0x00);
    }

    #[test]
    fn op_jp_jumps_to_addr() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::JP(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.pc, 0x0aaa);
    }

    #[test]
    fn op_call_calls_addr() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.pc, 0x0aaa);
        assert_eq!(registers.sp, 0x0002);
        assert_eq!(memory.get_u16(0x0002), 0x0200);
    }

    #[test]
    fn op_se_vkk_increments_pc_if_v_equals_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vkk_does_not_increment_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_does_increment_pc_if_v_equals_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SNE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_increments_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SNE(0x08, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_increments_pc_if_vx_equals_vy() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0a, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SEV(0x08, 0x0a)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_does_not_increment_pc_if_vx_not_equal_to_vy() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0a, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x0a)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_ld_vkk_sets_register_v_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0x66)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0x66);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 30)
            .call(&mut registers, &mut memory)
            .expect("Failed to set register");
        Ops::ADD(0, 12)
            .call(&mut registers, &mut memory)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, 42);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v_no_carry() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 200)
            .call(&mut registers, &mut memory)
            .expect("Failed to set register");
        Ops::ADD(0, 200)
            .call(&mut registers, &mut memory)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, u8::MAX);
        assert_eq!(registers.vf, 0); // CARRY FLAG
    }
}
