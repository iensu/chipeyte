pub mod instruction_decoder;
pub mod registers;

use crate::cpu::instruction_decoder::decode;
use crate::cpu::registers::Registers;
use crate::interface;
use crate::memory::Memory;
use crate::{errors::ChipeyteError, operations::Ops};
use std::fmt::Display;

pub const PROGRAM_START: u16 = 0x0200;
pub const INSTRUCTION_LENGTH: u16 = 2;

#[derive(Debug, PartialEq)]
pub struct CPU {
    pub counter: u32,
    pub registers: Registers,
}

impl CPU {
    pub fn new(initial_pc: u16) -> CPU {
        CPU {
            counter: 0,
            registers: Registers::new(initial_pc),
        }
    }

    pub fn tick(
        &mut self,
        memory: &mut Memory,
        screen: &mut dyn interface::Drawable,
        controller: &mut dyn interface::Controllable,
    ) -> Result<(u16, Ops), ChipeyteError> {
        let instruction = self.fetch(memory);

        let operation = decode(instruction);

        if instruction == 0 {
            return Ok((self.registers.pc, Ops::UNKNOWN(instruction)));
        }

        self.registers.pc += INSTRUCTION_LENGTH;
        self.execute(operation, memory, screen, controller)?;

        Ok((self.registers.pc, operation))
    }

    fn fetch(&self, memory: &Memory) -> u16 {
        memory.get_u16(self.registers.pc.into())
    }

    fn execute(
        &mut self,
        operation: Ops,
        memory: &mut Memory,
        screen: &mut dyn interface::Drawable,
        controller: &mut dyn interface::Controllable,
    ) -> Result<(), ChipeyteError> {
        operation.call(&mut self.registers, memory, screen, controller)
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
