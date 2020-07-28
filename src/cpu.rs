use crate::cpu::instruction_decoder::{decode, Ops};
use crate::cpu::registers::Registers;
use crate::memory::Memory;

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

    pub fn tick(&self, memory: &mut Memory) {
        let instruction = self.fetch(memory);
        let operation = decode(instruction);
        self.execute(operation, memory);
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc.into())
    }

    fn execute(&self, _operation: Ops, _memory: &mut Memory) {
        todo!();
    }
}

mod instruction_decoder;
mod registers;
