use super::*;

use std::cell::Cell;

pub struct Sprite<'t> {
    texture: Rc<EngineTex<'t>>,
    sprite_state: Cell<usize>,
    sprite_sheet: Rc<SpriteSheet>,
}

impl<'t> Sprite<'t> {
    pub fn new(
        texture: &Rc<EngineTex<'t>>,
        sprite_state: usize,
        sprite_sheet: &Rc<SpriteSheet>
    ) -> Self {
        Self {
            texture: texture.clone(),
            sprite_state: sprite_state.into(),
            sprite_sheet: sprite_sheet.clone()
        }
    }
}

impl<'t> Clone for Sprite<'t> {
    fn clone(&self) -> Self {
        Sprite::new(
            &self.texture,
            self.sprite_state.get(),
            &self.sprite_sheet
        )
    }
}

pub trait HasSprite {
    fn get_texture(&self) -> &EngineTex<'_>;
    fn get_src_rect(&self) -> Option<Rect>;
}

impl<'t> HasSprite for Sprite<'t> {
    fn get_texture(&self) -> &EngineTex<'t> {
        &self.texture
    }
    fn get_src_rect(&self) -> Option<Rect> {
        self.sprite_sheet[self.sprite_state.get()]
    }
}
