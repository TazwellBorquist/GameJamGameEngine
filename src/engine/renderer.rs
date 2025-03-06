extern crate sdl2;

use sdl2::{
    Sdl,
    VideoSubsystem,
    video::{
        Window
    },
    render::{
        Canvas,
        Texture
    },
    rect::{
        Rect
    },
};

use std::cell::RefCell;
use std::rc::Rc;

/// Renderer is responsible for window creation, and drawing onto the canvas
pub struct Renderer {
    pub size: (u32, u32),
    pub video_sys: VideoSubsystem,
    pub canvas: RefCell<Canvas<Window>>,
}

impl Renderer {
    pub fn new(sdl_context: &Sdl, height: u32, width: u32) -> Self {
        let video_sys = sdl_context.video().unwrap();

        let window = video_sys.window("GameJamEngine", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .accelerated()
            .build()
            .unwrap();

        Self {
            size: (width, height),
            video_sys: video_sys,
            canvas: RefCell::new(canvas),
        }
    }

    pub fn draw(&self, drawable: &impl Drawable) {
        let _ = self.canvas.borrow_mut().copy(
            drawable.get_texture().as_ref(),
            drawable.get_src_rect(),
            drawable.get_dst_rect()
        );
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}

pub trait Drawable {
    fn get_src_rect(&self) -> Option<Rect>;
    fn get_dst_rect(&self) -> Option<Rect>;
    fn get_texture(&self) -> Rc<Texture<'_>>;
}