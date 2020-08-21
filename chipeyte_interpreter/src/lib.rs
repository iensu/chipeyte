use cpu::{ProgramState, CPU, PROGRAM_START};
use interface::{Audible, Controllable, Drawable, UserAction};
use memory::Memory;
use std::{
    thread,
    time::{Duration, SystemTime},
};

pub struct Config {
    pub clock_speed: Option<Duration>,
}

impl Config {
    pub fn new(clock_speed: Option<Duration>) -> Self {
        Self { clock_speed }
    }

    pub fn default() -> Self {
        Self::new(None)
    }
}

pub struct ChipeyteInterpreter {
    cpu: CPU,
    memory: Memory,
    clock_speed: Duration,
}

impl ChipeyteInterpreter {
    pub fn new(config: Config) -> Self {
        env_logger::init();

        Self {
            cpu: CPU::new(PROGRAM_START),
            memory: Memory::new(),
            clock_speed: config.clock_speed.unwrap_or(Duration::new(0, 1_500_000)),
        }
    }

    pub fn run(
        &mut self,
        screen: &mut dyn Drawable,
        speaker: &dyn Audible,
        controller: &mut dyn Controllable,
        program: &Vec<u8>,
    ) {
        let timer_duration = Duration::new(0, 16_700_000);

        let mut timer_clock = SystemTime::now();

        self.memory.load_program(PROGRAM_START.into(), program);

        'running: loop {
            let start_time = SystemTime::now();

            match screen.poll_events() {
                Some(UserAction::Quit) => break 'running,
                Some(UserAction::KeyDown(Some(key))) => controller.press_key(key),
                Some(UserAction::KeyUp(Some(key))) => controller.release_key(key),
                _ => {}
            };

            match self.cpu.tick(&mut self.memory, screen, controller) {
                Ok(ProgramState::End) => {
                    log::info!("Reached program end");
                    break 'running;
                }
                Err(e) => {
                    panic!("Something went wrong: {:?}", e);
                }
                _ => {}
            };

            if self.cpu.registers.st > 0 && !speaker.is_playing() {
                speaker.play_sound();
            } else if self.cpu.registers.st < 1 && speaker.is_playing() {
                speaker.stop_sound();
            }

            if let Ok(elapsed) = timer_clock.elapsed() {
                if elapsed > timer_duration {
                    if self.cpu.registers.dt > 0 {
                        self.cpu.registers.dt -= 1;
                    }

                    if self.cpu.registers.st > 0 {
                        self.cpu.registers.st -= 1;
                    }

                    timer_clock = SystemTime::now();
                }
            }

            match start_time.elapsed() {
                Ok(elapsed) => {
                    if elapsed < self.clock_speed {
                        thread::sleep(self.clock_speed - elapsed);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    break 'running;
                }
            }
        }

        log::debug!("{}", self.memory);
        log::debug!("\n{}", self.cpu);
    }
}

mod cpu;
pub mod errors;
pub mod interface;
mod memory;
mod operations;
mod types;