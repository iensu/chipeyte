use std::collections::HashSet;

mod sdl2;

pub use crate::graphics::sdl2::Sdl2Screen;

#[derive(Debug)]
pub enum UserAction {
    Quit,
    KeyDown(Option<u8>),
    KeyUp(Option<u8>),
}

pub trait Drawable {
    fn clear(&mut self);

    fn add_pixel(&mut self, x: u8, y: u8);

    fn remove_pixel(&mut self, x: u8, y: u8);

    fn has_pixel(&self, x: u8, y: u8) -> bool;

    fn render(&mut self);

    fn poll_events(&mut self) -> Option<UserAction>;

    fn get_pixels(&self) -> HashSet<(u8, u8)>;
}

pub trait Audible {
    fn play_sound(&mut self);

    fn stop_sound(&mut self);

    fn is_playing(&self) -> bool;
}

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);
