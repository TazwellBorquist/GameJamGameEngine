extern crate sdl2;

use sdl2::rect::Rect;
use sdl2::render::Texture;

use std::rc::Rc;
use std::cell::{Ref, RefCell};

use crate::engine::renderer;

pub struct Sprite<'r> {
    dst_rect: RefCell<Rect>,

    curr_src_rect: RefCell<usize>,
    src_rect_list: Vec<Option<Rect>>,

    texture: Rc<Texture<'r>>,
}

impl<'r> Sprite<'r> {
    pub fn new(size: (u32, u32), texture: Rc<Texture<'r>>) -> Self {
        Self {
            dst_rect: RefCell::new(Rect::new(0, 0, size.0, size.1)),
            curr_src_rect: RefCell::new(0),
            src_rect_list: vec![None],
            texture: texture.clone(),
        }
    }

    pub fn set_dst_rect(&self, x: Option<i32>, y: Option<i32>, width: Option<u32>, height: Option<u32>) {
        let mut self_rect = self.dst_rect.borrow_mut();
        if let Some(x) = x { self_rect.set_x(x); }
        if let Some(y) = y { self_rect.set_y(y); }
        if let Some(width) = width { self_rect.set_width(width); }
        if let Some(height) = height { self_rect.set_height(height); }
    }
}

impl renderer::Drawable for Sprite<'_> {
    fn get_dst_rect(&self) -> Option<Rect> {
        Some(*self.dst_rect.borrow())
    }

    fn get_src_rect(&self) -> Option<Rect> {
        self.src_rect_list[*self.curr_src_rect.borrow()]
    }

    fn get_texture(&self) -> Rc<Texture<'_>> {
        self.texture.clone()
    }
}