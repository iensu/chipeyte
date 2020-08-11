use super::{Color, Drawable, UserAction};
use glfw::{Action, Context, Key, WindowEvent};
use luminance::framebuffer::Framebuffer;
use luminance::{context::GraphicsContext, pipeline::PipelineState, texture::Dim2};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

pub struct LumiCanvas {
    fg_color: Color,
    bg_color: Color,
    surface: GlfwSurface,
    back_buffer: Framebuffer<GL33, Dim2, (), ()>,
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        let r = self.0 as f32;
        let g = self.1 as f32;
        let b = self.2 as f32;

        [r / 255.0, g / 255.0, b / 255.0, 1.0]
    }
}

impl LumiCanvas {
    pub fn init(fg_color: Color, bg_color: Color) -> Self {
        let dim = WindowDim::Windowed {
            width: 960,
            height: 540,
        };
        let mut surface =
            GlfwSurface::new_gl33("Chipeyte", WindowOpt::default().set_dim(dim)).unwrap();

        let back_buffer = surface.back_buffer().unwrap();

        let color: [f32; 4] = bg_color.clone().into();

        log::debug!("{:?}", color);

        let render = surface
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                |_, _| Ok(()),
            )
            .assume();

        if render.is_ok() {
            log::debug!("Render was OK!");
            surface.window.swap_buffers();
        }

        LumiCanvas {
            fg_color,
            bg_color,
            surface,
            back_buffer,
        }
    }
}

impl Drawable for LumiCanvas {
    fn clear(&mut self) {}

    fn draw(&mut self, _x: u8, _y: u8) {}

    fn poll_events(&mut self) -> Option<UserAction> {
        self.surface.window.glfw.poll_events();

        self.surface
            .events_rx
            .try_iter()
            .fold(None, |result, (_, event)| {
                result.or_else(move || match event {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                        Some(UserAction::Quit)
                    }
                    _ => None,
                })
            })
    }

    fn get_pixels(&self) -> Vec<(u8, u8)> {
        vec![]
    }
}
