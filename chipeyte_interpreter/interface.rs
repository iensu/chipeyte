use std::collections::HashSet;

pub mod sdl2;

#[derive(Debug)]
pub enum UserAction {
    Quit,
    KeyDown(Option<u8>),
    KeyUp(Option<u8>),
}

// SCREEN

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);

pub trait Drawable {
    fn clear(&mut self);

    fn add_pixel(&mut self, x: u8, y: u8);

    fn remove_pixel(&mut self, x: u8, y: u8);

    fn has_pixel(&self, x: u8, y: u8) -> bool;

    fn render(&mut self);

    fn poll_events(&mut self) -> Option<UserAction>;

    fn get_pixels(&self) -> HashSet<(u8, u8)>;
}

// AUDIO

pub trait Audible {
    fn play_sound(&mut self);

    fn stop_sound(&mut self);

    fn is_playing(&self) -> bool;
}

// CONTROLLER

pub trait Controllable {
    fn press_key(&mut self, key: u8);

    fn release_key(&mut self, key: u8);

    fn is_pressed(&self, key: u8) -> bool;

    fn get_pressed_key(&mut self) -> Option<u8>;
}

pub struct Controller {
    pressed_keys: HashSet<u8>,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            pressed_keys: HashSet::new(),
        }
    }
}

impl Controllable for Controller {
    fn press_key(&mut self, key: u8) {
        self.pressed_keys.insert(key);
    }

    fn release_key(&mut self, key: u8) {
        self.pressed_keys.remove(&key);
    }

    fn is_pressed(&self, key: u8) -> bool {
        self.pressed_keys.contains(&key)
    }

    fn get_pressed_key(&mut self) -> Option<u8> {
        if self.pressed_keys.is_empty() {
            None
        } else {
            let keys = self.pressed_keys.iter().cloned().collect::<Vec<u8>>();
            let key = keys.first().unwrap();
            self.pressed_keys.remove(key);
            Some(*key)
        }
    }
}
