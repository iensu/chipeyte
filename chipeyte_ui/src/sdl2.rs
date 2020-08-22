use chipeyte_interpreter::interface::{Audible, Color, Controller, Drawable, UserAction};
use sdl2::{
    self,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color as Sdl2Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    EventPump, Sdl,
};
use std::collections::HashSet;

pub struct Sdl2UI {
    pub screen: Sdl2Screen,
    pub speaker: Sdl2Speaker,
    pub controller: Controller,
}

impl Sdl2UI {
    pub fn init(fg_color: Color, bg_color: Color) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let screen = Sdl2Screen::init(&sdl_context, fg_color, bg_color);
        let speaker = Sdl2Speaker::init(&sdl_context);
        let controller = Controller::new();

        Self {
            screen,
            speaker,
            controller,
        }
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
    pub fn init(sdl_context: &Sdl, fg_color: Color, bg_color: Color) -> Sdl2Screen {
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
        let Color(r, g, b) = bg_color;
        let background_color = Sdl2Color::RGB(r, g, b);

        canvas.set_draw_color(background_color);
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
        let Color(r, g, b) = self.bg_color;

        self.pixels.clear();
        self.canvas.set_draw_color(Sdl2Color::RGB(r, g, b));
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
        let Color(r, g, b) = self.bg_color;
        self.canvas.set_draw_color(Sdl2Color::RGB(r, g, b));
        self.canvas.clear();

        let Color(r, g, b) = self.fg_color;
        self.canvas.set_draw_color(Sdl2Color::RGB(r, g, b));

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

/// Translate Sdl2 keycode to Chipeyte key.
///
/// Original Chip-8 layout
///
///   ,---------------.
///   | 1 | 2 | 3 | C |
///   |---|---|---|---|
///   | 4 | 5 | 6 | D |
///   |---|---|---|---|
///   | 7 | 8 | 9 | E |
///   |---|---|---|---|
///   | A | 0 | B | F |
///   `---------------´
///
/// Modern keyboard layout:
///
///   ,---------------.
///   | 6 | 7 | 8 | 9 |
///   |---|---|---|---|
///   | Y | U | I | O |
///   |---|---|---|---|
///   | H | J | K | L |
///   |---|---|---|---|
///   | N | M | , | . |
///   `---------------´
fn translate_key(key: &Keycode) -> Option<u8> {
    match key {
        Keycode::Num6 => Some(1),
        Keycode::Num7 => Some(2),
        Keycode::Num8 => Some(3),
        Keycode::Num9 => Some(12),

        Keycode::Y => Some(4),
        Keycode::U => Some(5),
        Keycode::I => Some(6),
        Keycode::O => Some(13),

        Keycode::H => Some(7),
        Keycode::J => Some(8),
        Keycode::K => Some(9),
        Keycode::L => Some(14),

        Keycode::N => Some(10),
        Keycode::M => Some(0),
        Keycode::Comma => Some(11),
        Keycode::Period => Some(15),
        _ => None,
    }
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

pub struct Sdl2Speaker {
    audio_device: AudioDevice<SquareWave>,
}

impl Sdl2Speaker {
    pub fn init(sdl_context: &Sdl) -> Self {
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

        Self { audio_device }
    }
}

impl Audible for Sdl2Speaker {
    fn play_sound(&self) {
        self.audio_device.resume();
    }

    fn stop_sound(&self) {
        self.audio_device.pause();
    }

    fn is_playing(&self) -> bool {
        match self.audio_device.status() {
            sdl2::audio::AudioStatus::Playing => true,
            _ => false,
        }
    }
}
