use crate::cpu::error::CPUError;
use crate::cpu::instruction_decoder::{decode, Ops};
use crate::cpu::registers::{NumericRegister, Registers};
use crate::memory::Memory;
use std::convert::TryFrom;

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
        self.execute(operation, memory)
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc.into())
    }

    fn execute(&mut self, operation: Ops, memory: &mut Memory) -> Result<(), CPUError> {
        match operation {
            Ops::SYS(_) => Ok(()),

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

            Ops::RET => {
                self.registers.pc = memory.get_u16(self.registers.sp.into());
                self.registers.sp -= 1;
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

    #[test]
    fn op_0nnn_is_ignored() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, 0x0200);

        memory.set_u16(0x0200, 0x0aaa);
        cpu.tick(&mut memory);

        println!("{:?}", cpu.registers);

        assert_eq!(cpu.registers, Registers::new(0x0200));

        let mut expected_memory = Memory::new();
        expected_memory.set_u16(0x0200, 0x0aaa);

        assert_eq!(memory, expected_memory);
    }

    #[test]
    fn op_00ee_returns() {
        todo!();
    }

    #[test]
    fn op_1nnn_jumps_to_addr() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, 0x0200);

        memory.set_u16(0x0200, 0x1aaa);
        cpu.tick(&mut memory);

        println!("{:?}", cpu.registers);

        assert_eq!(cpu.registers.pc, 0x0aaa);
    }

    #[test]
    fn op_2nnn_calls_addr() {
        todo!();
    }

    #[test]
    fn op_6nnn_loads_register() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, 0x0200);

        memory.set_u16(0x0200, 0x6042);
        cpu.tick(&mut memory);

        println!("{:?}", cpu.registers);

        assert_eq!(cpu.registers.v0, 0x42);
    }
}
