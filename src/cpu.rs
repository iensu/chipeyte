pub mod instruction_decoder;
pub mod registers;

use crate::cpu::instruction_decoder::decode;
use crate::memory::Memory;
use crate::Registers;
use crate::{operations::Callable, ChipeyteError, Ops};
use std::fmt::Display;

pub const PROGRAM_START: u16 = 0x0200;
pub const INSTRUCTION_LENGTH: u16 = 2;

#[derive(Debug, PartialEq)]
pub struct CPU {
    pub counter: u32,
    pub registers: Registers,
}

impl CPU {
    pub fn new(interrupt_period: u32, initial_pc: u16) -> CPU {
        CPU {
            counter: interrupt_period,
            registers: Registers::new(initial_pc),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> Result<(), ChipeyteError> {
        let instruction = self.fetch(memory);
        let operation = decode(instruction);

        log::info!(
            "{:04x?}: {:x?} - {:?}",
            self.registers.pc,
            instruction,
            operation
        );

        self.registers.pc += INSTRUCTION_LENGTH;
        self.execute(operation, memory)
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc.into())
    }

    fn execute(&mut self, operation: Ops, memory: &mut Memory) -> Result<(), ChipeyteError> {
        operation.call(&mut self.registers, memory)
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "
Counter: {:04x?}
PC: {:04x?} SP: {:04x?} I: {:04x?}
DT: {:02x?}   ST: {:02x?}

     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
  ,-------------------------------------------------,
V | {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} {:02x?} |
  `-------------------------------------------------´",
            self.counter,
            self.registers.pc,
            self.registers.sp,
            self.registers.i,
            self.registers.dt,
            self.registers.st,
            self.registers.v0,
            self.registers.v1,
            self.registers.v2,
            self.registers.v3,
            self.registers.v4,
            self.registers.v5,
            self.registers.v6,
            self.registers.v7,
            self.registers.v8,
            self.registers.v9,
            self.registers.va,
            self.registers.vb,
            self.registers.vc,
            self.registers.vd,
            self.registers.ve,
            self.registers.vf,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_increments_pc_during_tick() {
        let mut memory = Memory::new();
        let mut cpu = CPU::new(0, PROGRAM_START);

        let program = vec![0x0aaa, 0x0aaa, 0x0aaa];

        memory.load_program(PROGRAM_START.into(), &program);

        assert_eq!(cpu.registers.pc, PROGRAM_START);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.pc, PROGRAM_START + INSTRUCTION_LENGTH * 2);

        cpu.tick(&mut memory).unwrap();

        assert_eq!(cpu.registers.pc, PROGRAM_START + INSTRUCTION_LENGTH * 3);
    }
}
