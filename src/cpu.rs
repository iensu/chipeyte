use crate::cpu::error::CPUError;
use crate::cpu::instruction_decoder::{decode, Ops};
use crate::cpu::registers::{NumericRegister, Registers};
use crate::memory::Memory;
use std::convert::TryFrom;

const INSTRUCTION_LENGTH: u16 = 2;

#[derive(Debug)]
pub struct CPU {
    pub counter: u32,
    registers: Registers,
}

impl CPU {
    pub fn new(interrupt_period: u32, initial_pc: u16) -> CPU {
        CPU {
            counter: interrupt_period,
            registers: Registers::new(initial_pc),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> Result<(), CPUError> {
        let instruction = self.fetch(memory);
        let operation = decode(instruction);

        self.registers.pc += INSTRUCTION_LENGTH;
        self.execute(operation, memory)
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc.into())
    }

    fn execute(&mut self, operation: Ops, memory: &mut Memory) -> Result<(), CPUError> {
        match operation {
            Ops::SYS(_) => Ok(()),

            Ops::RET => {
                self.registers.pc = memory.get_u16(self.registers.sp.into());
                self.registers.sp -= 1;
                Ok(())
            }

            Ops::JP(addr) => {
                self.registers.pc = addr;
                Ok(())
            }

            Ops::CALL(addr) => {
                self.registers.sp += 1;
                memory.set_u16(self.registers.sp.into(), self.registers.pc);
                self.registers.pc = addr;
                Ok(())
            }

            Ops::SE(v, byte) => {
                let register = NumericRegister::try_from(v)?;
                let value = self.registers.get_numeric_register(register);

                if value == byte {
                    self.registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SNE(v, byte) => {
                let register = NumericRegister::try_from(v)?;
                let value = self.registers.get_numeric_register(register);

                if value != byte {
                    self.registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SEV(vx, vy) => {
                let reg_x = NumericRegister::try_from(vx)?;
                let reg_y = NumericRegister::try_from(vy)?;
                let x = self.registers.get_numeric_register(reg_x);
                let y = self.registers.get_numeric_register(reg_y);

                if x == y {
                    self.registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LD(v, byte) => {
                let register = NumericRegister::try_from(v)?;

                self.registers.set_numeric_register(register, byte);
                Ok(())
            }

            _ => Err(CPUError::OpNotImplemented(operation)),
        }
    }
}

mod error;
pub mod instruction_decoder;
mod registers;

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM_START: u16 = 0x0200;
    type Program = Vec<(u16, u16)>;

    #[test]
    fn op_0nnn_is_ignored() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        memory.set_u16(PROGRAM_START.into(), 0x0aaa);
        cpu.tick(&mut memory).unwrap();

        assert_eq!(
            cpu.registers,
            Registers::new(PROGRAM_START + INSTRUCTION_LENGTH)
        );

        let mut expected_memory = Memory::new();
        expected_memory.set_u16(PROGRAM_START.into(), 0x0aaa);

        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn op_00ee_returns() {
        let program: Program = vec![
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
            (PROGRAM_START, 0x6842),                      // LOAD DATA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x3842), // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 3);
    }

    #[test]
    fn op_3xkk_does_not_skip_instruction_if_not_equal() {
        let program: Program = vec![
            (PROGRAM_START, 0x6842),                      // LOAD DATA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x38aa), // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 2);
    }

    #[test]
    fn op_4xkk_does_not_skip_instruction_if_equal() {
        let program: Program = vec![
            (PROGRAM_START, 0x6842),                      // LOAD DATA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x4842), // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 2);
    }

    #[test]
    fn op_4xkk_skips_instruction_if_not_equal() {
        let program: Program = vec![
            (PROGRAM_START, 0x6842),                      // LOAD DATA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x48aa), // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        cpu.tick(&mut memory).unwrap();
        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 3);
    }

    #[test]
    fn op_5xkk_skips_instruction_if_equal() {
        let program: Program = vec![
            (PROGRAM_START, 0x6a42),                          // LOAD DATA VA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x6b42),     // LOAD DATA VB
            (PROGRAM_START + INSTRUCTION_LENGTH * 2, 0x5ab0), // SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

        (0..program.len()).for_each(|_| {
            cpu.tick(&mut memory).unwrap();
        });

        assert_eq!(cpu.registers.sp, 0);
        assert_eq!(cpu.registers.pc, 0x0200 + INSTRUCTION_LENGTH * 4);
    }

    #[test]
    fn op_5xkk_does_not_skip_instruction_if_not_equal() {
        let program: Program = vec![
            (PROGRAM_START, 0x6a42),                          // LOAD DATA VA
            (PROGRAM_START + INSTRUCTION_LENGTH, 0x6b21),     // LOAD DATA VB
            (PROGRAM_START + INSTRUCTION_LENGTH * 2, 0x5ab0), // DO NOT SKIP
        ];

        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        program.iter().for_each(|(loc, op)| {
            memory.set_u16((*loc).into(), *op);
        });

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
}
