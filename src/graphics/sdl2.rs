use super::{Color, Drawable, UserAction};
use sdl2::{
    self, event::Event, keyboard::Keycode, rect::Rect, render::Canvas, video::Window, EventPump,
};
use std::collections::HashSet;

impl Into<sdl2::pixels::Color> for Color {
    fn into(self) -> sdl2::pixels::Color {
        sdl2::pixels::Color::RGB(self.0, self.1, self.2)
    }
}

pub struct Sdl2Screen {
    event_pump: EventPump,
    canvas: Canvas<Window>,
    bg_color: Color,
    fg_color: Color,
    pixels: HashSet<(u8, u8)>,
    pixel_size: u32,
}

impl Sdl2Screen {
    pub fn init(fg_color: Color, bg_color: Color) -> Sdl2Screen {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        // 64 x 32 pixel grid
        let pixel_size = 10;
        let width = 64 * pixel_size;
        let height = 32 * pixel_size;

        let window = video_subsystem
            .window("Chipeyte", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();

        canvas.set_draw_color(bg_color.clone());
        canvas.clear();
        canvas.present();

        Sdl2Screen {
            canvas,
            event_pump,
            fg_color,
            bg_color,
            pixel_size,
            pixels: HashSet::new(),
        }
    }
}

impl Drawable for Sdl2Screen {
    fn clear(&mut self) {
        self.pixels.clear();
        self.canvas.set_draw_color(self.bg_color.clone());
        self.canvas.clear();
        self.canvas.present();
    }

    fn remove_pixel(&mut self, x: u8, y: u8) {
        self.pixels.remove(&(x, y));
    }

    fn has_pixel(&self, x: u8, y: u8) -> bool {
        self.pixels.contains(&(x, y))
    }

    fn add_pixel(&mut self, x: u8, y: u8) {
        self.pixels.insert((x, y));
    }

    fn render(&mut self) {
        self.canvas.set_draw_color(self.bg_color.clone());
        self.canvas.clear();

        self.canvas.set_draw_color(self.fg_color.clone());
        for pixel in self.pixels.iter() {
            let pos_x = pixel.0 as i32 * self.pixel_size as i32;
            let pos_y = pixel.1 as i32 * self.pixel_size as i32;

            if let Err(e) =
                self.canvas
                    .fill_rect(Rect::new(pos_x, pos_y, self.pixel_size, self.pixel_size))
            {
                eprintln!("Failed to draw pixel: {:?}", e);
            }
        }
        self.canvas.present();
    }

    fn poll_events(&mut self) -> Option<UserAction> {
        self.event_pump.poll_iter().fold(None, |result, event| {
            result.or_else(move || match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Some(UserAction::Quit);
                }
                _ => {
                    return None;
                }
            })
        })
    }

    fn get_pixels(&self) -> HashSet<(u8, u8)> {
        self.pixels.clone()
    }
}
