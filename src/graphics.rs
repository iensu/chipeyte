mod lumi;
mod sdl2;

pub use crate::graphics::lumi::LumiCanvas;
pub use crate::graphics::sdl2::Sdl2Canvas;

#[derive(Debug)]
pub enum UserAction {
    Quit,
}

pub trait Drawable {
    fn clear(&mut self);

    fn draw(&mut self, x: u8, y: u8);

    fn poll_events(&mut self) -> Option<UserAction>;

    fn get_pixels(&self) -> Vec<(u8, u8)>;
}

#[derive(Clone)]
pub struct Color(pub u8, pub u8, pub u8);
