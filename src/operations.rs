use crate::{cpu::registers::NumericRegister, types::*, ChipeyteError, Memory, Registers};
use std::convert::TryFrom;

const INSTRUCTION_LENGTH: u16 = 2;

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
                registers.sp -= 1;
                Ok(())
            }

            Ops::JP(addr) => {
                registers.pc = *addr;
                Ok(())
            }

            Ops::CALL(addr) => {
                registers.sp += 1;
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
    use crate::CPU;

    const PROGRAM_START: u16 = 0x0200;
    const INSTRUCTION_LENGTH: u16 = 2;
    type Program = Vec<u16>;

    #[test]
    fn op_0nnn_is_ignored() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.set_u16(PROGRAM_START.into(), 0x0aaa);
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu, CPU::new(0, PROGRAM_START + INSTRUCTION_LENGTH));

        let mut expected_memory = Memory::new();
        expected_memory.set_u16(PROGRAM_START.into(), 0x0aaa);

        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn op_00ee_returns() {
        let program: Vec<(u16, u16)> = vec![
            (PROGRAM_START, 0x2aaa), // CALL 0aaa
            (0x0aaa, 0x00ee),        // RETURN
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);

        let mut expected_memory = Memory::new();
        program.iter().for_each(|(loc, operation)| {
            expected_memory.set_u16((*loc).into(), *operation);
        });

        expected_memory.set_u16(0x0001, PROGRAM_START + INSTRUCTION_LENGTH);

        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn op_1nnn_jumps_to_addr() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.set_u16(PROGRAM_START.into(), 0x1aaa);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.pc, 0x0aaa);
    }

    #[test]
    fn op_2nnn_calls_addr() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.set_u16(PROGRAM_START.into(), 0x2aaa);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 1);
        assert_eq!(cpu.registers.pc, 0x0aaa);

        let mut expected_memory = Memory::new();
        expected_memory.set_u16(PROGRAM_START.into(), 0x2aaa);
        expected_memory.set_u16(0x0001, PROGRAM_START + INSTRUCTION_LENGTH);

        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn op_3xkk_skips_instruction_if_equal() {
        let program: Program = vec![
            0x6842, // LOAD DATA
            0x3842, // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 3);
    }

    #[test]
    fn op_3xkk_does_not_skip_instruction_if_not_equal() {
        let program: Program = vec![
            0x6842, // LOAD DATA
            0x38aa, // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 2);
    }

    #[test]
    fn op_4xkk_does_not_skip_instruction_if_equal() {
        let program: Program = vec![
            0x6842, // LOAD DATA
            0x4842, // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 2);
    }

    #[test]
    fn op_4xkk_skips_instruction_if_not_equal() {
        let program: Program = vec![
            0x6842, // LOAD DATA
            0x48aa, // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 3);
    }

    #[test]
    fn op_5xkk_skips_instruction_if_equal() {
        let program: Vec<u16> = vec![
            0x6a42, // LOAD DATA VA
            0x6b42, // LOAD DATA VB
            0x5ab0, // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        (0..program.len()).for_each(|_| {
            cpu.tick(&mut memory).unwrap();
        });

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 4);
    }

    #[test]
    fn op_5xkk_does_not_skip_instruction_if_not_equal() {
        let program: Program = vec![
            0x6a42, // LOAD DATA VA
            0x6b21, // LOAD DATA VB
            0x5ab0, // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.load_program(&program);

        (0..program.len()).for_each(|_| {
            cpu.tick(&mut memory).unwrap();
        });

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 3);
    }

    #[test]
    fn op_6nnn_loads_register() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.set_u16(PROGRAM_START.into(), 0x6042);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.v0, 0x42);
    }

    #[test]
    fn op_7xkk_adds_kk_to_vx() {
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
    fn op_7xkk_adds_kk_to_vx_no_carry() {
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
