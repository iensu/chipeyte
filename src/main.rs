//! An emulation of the Chip-8 programming langauge

mod cpu;
mod errors;
mod interface;
mod memory;
mod operations;
mod program_reader;
mod types;

pub use cpu::registers::Registers;
pub use errors::ChipeyteError;
pub use interface::Drawable;
pub use memory::Memory;
pub use operations::Ops;

use cpu::{ProgramState, CPU};
use interface::{sdl2::Sdl2Screen, Audible, Color, Controllable, Controller, UserAction};
use std::env;
use std::{
    path::Path,
    thread,
    time::{Duration, SystemTime},
};

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

    let target_framerate = Duration::new(0, 1_500_000);
    let timer_duration = Duration::new(0, 16_700_000);
    let mut timer_clock = SystemTime::now();

    'running: loop {
        let start_time = SystemTime::now();

        match screen.poll_events() {
            Some(UserAction::Quit) => break 'running,
            Some(UserAction::KeyDown(Some(key))) => controller.press_key(key),
            Some(UserAction::KeyUp(Some(key))) => controller.release_key(key),
            _ => {}
        };

        match cpu.tick(&mut memory, &mut screen, &mut controller) {
            Ok(ProgramState::End) => {
                log::info!("Reached program end");
                break 'running;
            }
            Err(e) => {
                panic!("Something went wrong: {:?}", e);
            }
            _ => {}
        };

        if cpu.registers.st > 0 && !screen.is_playing() {
            screen.play_sound();
        } else if cpu.registers.st < 1 && screen.is_playing() {
            screen.stop_sound();
        }

        if let Ok(elapsed) = timer_clock.elapsed() {
            if elapsed > timer_duration {
                if cpu.registers.dt > 0 {
                    cpu.registers.dt -= 1;
                }
                if cpu.registers.st > 0 {
                    cpu.registers.st -= 1;
                }

                timer_clock = SystemTime::now();
            }
        }

        match start_time.elapsed() {
            Ok(elapsed) => {
                if elapsed < target_framerate {
                    thread::sleep(target_framerate - elapsed);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                break 'running;
            }
        }
    }

    log::debug!("{}", memory);
    log::debug!("\n{}", cpu);
}
