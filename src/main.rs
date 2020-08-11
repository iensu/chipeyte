//! An emulation of the Chip-8 programming langauge

mod cpu;
mod errors;
mod graphics;
mod memory;
mod operations;
mod program_reader;
mod types;

pub use cpu::registers::Registers;
pub use errors::ChipeyteError;
pub use graphics::Drawable;
pub use memory::Memory;
pub use operations::Ops;

use cpu::{CpuState, CPU};
use graphics::{Color, Sdl2Canvas as Canvas, UserAction};
use std::env;
use std::{path::Path, thread, time::Duration};

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to pass a file argument!");
    }

    let program = program_reader::read(Path::new(&args[1]));

    let mut cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    memory.load_program(cpu::PROGRAM_START.into(), &program);

    let fg_color = Color(255, 255, 255);
    let bg_color = Color(0, 0, 0);
    let mut canvas = Canvas::init(fg_color, bg_color);

    'running: loop {
        match canvas.poll_events() {
            Some(UserAction::Quit) => break 'running,
            None => {}
        };

        match cpu.tick(&mut memory, &mut canvas) {
            Ok(CpuState::End) => break 'running,
            Err(e) => eprintln!("Something went wrong: {:?}", e),
            _ => {}
        };

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    log::debug!("{}", memory);
    log::debug!("\n{}", cpu);
}
