//! An emulation of the Chip-8 programming langauge

use cpu::CPU;
use memory::Memory;

fn main() {
    let mut cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    if let Err(e) = cpu.tick(&mut memory) {
        println!("Something went wrong: {:?}", e);
    };
}

mod cpu;
mod memory;
