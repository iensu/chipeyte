use chipeyte_interpreter::interface;
use std::collections::HashSet;

pub struct MockUI {
    pub screen: Screen,
    pub speaker: Speaker,
    pub controller: interface::Controller,
}

impl MockUI {
    pub fn init(_fg_color: interface::Color, _bg_color: interface::Color) -> Self {
        Self {
            screen: Screen {
                pixels: HashSet::new(),
            },
            speaker: Speaker {},
            controller: interface::Controller::new(),
        }
    }
}

pub struct Screen {
    pixels: HashSet<(u8, u8)>,
}
pub struct Speaker {}

impl interface::Drawable for Screen {
    fn clear(&mut self) {
        self.pixels.clear();
    }
    fn add_pixel(&mut self, x: u8, y: u8) {
        self.pixels.insert((x, y));
    }
    fn remove_pixel(&mut self, x: u8, y: u8) {
        self.pixels.remove(&(x, y));
    }
    fn has_pixel(&self, x: u8, y: u8) -> bool {
        self.pixels.contains(&(x, y))
    }
    fn render(&mut self) {}

    fn poll_events(&mut self) -> Option<interface::UserAction> {
        None
    }
    fn get_pixels(&self) -> std::collections::HashSet<(u8, u8)> {
        self.pixels.clone()
    }
}

impl interface::Audible for Speaker {
    fn play_sound(&self) {}
    fn stop_sound(&self) {}
    fn is_playing(&self) -> bool {
        false
    }
}
