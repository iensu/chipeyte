//! An emulation of the Chip-8 programming langauge

mod controller;
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

use controller::{Controllable, Controller};
use cpu::{CpuState, CPU};
use graphics::{Color, Sdl2Screen, UserAction};
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
    let mut controller = Controller::new();

    memory.load_program(cpu::PROGRAM_START.into(), &program);

    let fg_color = Color(0, 255, 0);
    let bg_color = Color(0, 0, 0);
    let mut screen = Sdl2Screen::init(fg_color, bg_color);

    'running: loop {
        match screen.poll_events() {
            Some(UserAction::Quit) => break 'running,
            Some(UserAction::KeyDown(Some(key))) => controller.press_key(key),
            Some(UserAction::KeyUp(Some(key))) => controller.release_key(key),
            _ => {}
        };

        match cpu.tick(&mut memory, &mut screen, &controller) {
            Ok(CpuState::End) => break 'running,
            Err(e) => panic!("Something went wrong: {:?}", e),
            _ => {}
        };

        if cpu.registers.st > 0 {
            cpu.registers.st -= 1;
        }

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    log::debug!("{}", memory);
    log::debug!("\n{}", cpu);
}

// 48:0 57:9
//
