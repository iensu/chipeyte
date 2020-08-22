//! An emulation of the Chip-8 programming langauge

mod program_reader;

use chipeyte_interpreter::{interface::Color, ChipeyteInterpreter, Config};
use std::env;
use std::path::Path;

#[cfg(not(feature = "sdl2_ui"))]
use chipeyte_ui::mock::MockUI as UI;
#[cfg(feature = "sdl2_ui")]
use chipeyte_ui::sdl2::Sdl2UI as UI;

fn main() {
    #[cfg(feature = "logging")]
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to pass a file argument!");
    }

    let program = program_reader::read(Path::new(&args[1]));

    let mut ui = UI::init(Color(0, 255, 0), Color(0, 0, 0));

    let mut interpreter = ChipeyteInterpreter::new(Config::default());

    interpreter.run(&mut ui.screen, &ui.speaker, &mut ui.controller, &program);

    #[cfg(feature = "logging")]
    log::debug!("{}", interpreter);
}
