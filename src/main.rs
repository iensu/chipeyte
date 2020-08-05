//! An emulation of the Chip-8 programming langauge

mod cpu;
mod errors;
mod memory;
mod operations;
mod program_reader;
mod types;

pub use cpu::registers::Registers;
pub use errors::ChipeyteError;
pub use memory::Memory;
pub use operations::Ops;

use cpu::CPU;
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
