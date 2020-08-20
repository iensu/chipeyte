use super::{Audible, Color, Drawable, UserAction};
use sdl2::{
    self,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump,
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
    audio_device: AudioDevice<SquareWave>,
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
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

        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44_100),
            channels: Some(1),
            samples: None,
        };

        let audio_device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            })
            .unwrap();

        Sdl2Screen {
            canvas,
            event_pump,
            fg_color,
            bg_color,
            pixel_size,
            audio_device,
            pixels: HashSet::new(),
        }
    }
}

impl Audible for Sdl2Screen {
    fn play_sound(&mut self) {
        self.audio_device.resume();
    }

    fn stop_sound(&mut self) {
        self.audio_device.pause();
    }

    fn is_playing(&self) -> bool {
        match self.audio_device.status() {
            sdl2::audio::AudioStatus::Playing => true,
            _ => false,
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
                Event::KeyDown {
                    keycode: Some(key), ..
                } => return Some(UserAction::KeyDown(translate_key(&key))),
                Event::KeyUp {
                    keycode: Some(key), ..
                } => return Some(UserAction::KeyUp(translate_key(&key))),

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

fn translate_key(key: &Keycode) -> Option<u8> {
    match key {
        Keycode::X => Some(0),
        Keycode::Z => Some(1),
        Keycode::S => Some(2),
        Keycode::C => Some(3),
        Keycode::A => Some(4),
        Keycode::Space => Some(5),
        Keycode::D => Some(6),
        Keycode::Q => Some(7),
        Keycode::W => Some(8),
        Keycode::E => Some(9),
        Keycode::Num1 => Some(10),
        Keycode::Num2 => Some(11),
        Keycode::Num3 => Some(12),
        Keycode::Num4 => Some(13),
        Keycode::Num5 => Some(14),
        Keycode::Num6 => Some(15),
        _ => None,
    }
}
