//! An emulation of the Chip-8 programming langauge

mod program_reader;

use chipeyte_interpreter::{interface::Color, ChipeyteInterpreter, Config};
use chipeyte_sdl2::Sdl2Interface;
use std::env;
use std::path::Path;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Need to pass a file argument!");
    }

    let program = program_reader::read(Path::new(&args[1]));

    let mut interface = Sdl2Interface::init(Color(0, 255, 0), Color(0, 0, 0));

    let mut interpreter = ChipeyteInterpreter::new(Config::default());

    interpreter.run(
        &mut interface.screen,
        &interface.speaker,
        &mut interface.controller,
        &program,
    );

    log::debug!("{}", interpreter);
}
