use_gamespace!();
use crate::engine::assets::sprite::Sprite;

use super::entity::Entity;

use sdl2::render::Texture;
use sdl2::rect::Rect;

use std::rc::Rc;

pub struct EntitySpawner<'t> {
    pos: Position,
    center: Position,
    size: Size,

    is_visible: bool,

    sprite: Sprite<'t>,
}

impl<'t> EntitySpawner<'t> {
    pub fn new(
        pos: Position,
        center: Position,
        size: Size,
        is_visible: bool,
        sprite: Sprite<'t>,
    ) -> Self {
        Self {
            pos: pos,
            center: center,
            size: size,
            is_visible: is_visible,
            sprite: sprite
        }
    }

    pub fn spawn(&self) -> Entity<'_> {
        Entity::new(self.pos, self.center, self.size, self.is_visible, self.sprite.clone())
    }
}