//! An emulation of the Chip-8 programming langauge

use cpu::CPU;
use memory::Memory;
use std::env;
use std::path::Path;

const PROGRAM_START: usize = 0x0200;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to pass a file argument!");
    }

    let program = program_reader::read(Path::new(&args[1]));

    let mut cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    program.iter().enumerate().for_each(|(idx, op)| {
        memory.set_u16(PROGRAM_START + idx * 2, *op);
    });

    (0..program.len()).for_each(|_| {
        if let Err(e) = cpu.tick(&mut memory) {
            println!("Something went wrong: {:?}", e);
        };
    });
}

mod cpu;
mod memory;
mod program_reader;
