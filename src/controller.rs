use std::collections::HashSet;

pub trait Controllable {
    fn press_key(&mut self, key: u8);

    fn release_key(&mut self, key: u8);

    fn is_pressed(&self, key: u8) -> bool;

    fn get_pressed_key(&self) -> Option<u8>;
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
    fn get_pressed_key(&self) -> Option<u8> {
        if self.pressed_keys.is_empty() {
            None
        } else {
            let key = self.pressed_keys.iter().next().unwrap().clone();
            Some(key)
        }
    }
}
