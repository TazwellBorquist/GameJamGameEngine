use_gamespace!();

use sdl2::rect::Rect;

use std::cell::Cell;

use crate::engine::assets::{
    sprite::{HasSprite, Sprite},
    EngineTex,
};

pub struct Entity<'t> {
    pos: Cell<Position>,
    center: Cell<Position>,
    size: Cell<Size>,

    is_visible: Cell<bool>,
    sprite: Sprite<'t>,
}

impl<'t> Entity<'t> {
    pub fn new(
        pos: Position,
        center: Position,
        size: Size,
        is_visible: bool,
        sprite: Sprite<'t>,
    ) -> Self {
        Self {
            pos: pos.into(),
            center: center.into(),
            size: size.into(),
            is_visible: is_visible.into(),
            sprite: sprite,
        }
    }

    pub fn set_visible(&self, is_visible: bool) {
        self.is_visible.set(is_visible);
    }

    pub fn get_is_visible(&self) -> bool {
        self.is_visible.get()
    }
}

impl HasSprite for Entity<'_> {
    fn get_texture(&self) -> &EngineTex<'_> {
        self.sprite.get_texture()
    }
    fn get_src_rect(&self) -> Option<Rect> {
        self.sprite.get_src_rect()
    }
}

impl HasPosition for Entity<'_> {
    fn get_pos(&self) -> Position {
        self.pos.get()
    }
    fn set_pos(&self, pos: &Position) {
        self.pos.set(*pos);
    }
}

impl HasSize for Entity<'_> {
    fn get_size(&self) -> Size {
        self.size.get()
    }
    fn set_size(&self, size: &Size) {
        self.size.set(*size);
    }
}

impl HasGameRect for Entity<'_> {
    fn get_center(&self) -> Position {
        self.center.get()
    }
    fn set_center(&self, center: &Position) {
        self.center.set(*center);
    }
}
