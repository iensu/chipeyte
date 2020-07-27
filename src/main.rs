//! An emulation of the Chip-8 programming langauge

use cpu::CPU;
use memory::Memory;

fn main() {
    let cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    cpu.tick(&mut memory);
}

mod cpu;
mod memory;
