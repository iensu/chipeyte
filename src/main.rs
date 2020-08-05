//! An emulation of the Chip-8 programming langauge

use cpu::CPU;
use memory::Memory;
use std::env;
use std::path::Path;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to pass a file argument!");
    }

    let program = program_reader::read(Path::new(&args[1]));

    let mut cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    memory.load_program(&program);

    (0..program.len()).for_each(|_| {
        if let Err(e) = cpu.tick(&mut memory) {
            println!("Something went wrong: {:?}", e);
        };
    });

    log::debug!("{}", memory);
    log::debug!("\n{}", cpu);
}

mod cpu;
mod memory;
mod program_reader;
