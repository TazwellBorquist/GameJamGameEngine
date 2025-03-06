/// entities.rs
/// Collects and executes the behavior for entities through callbacks

use std::rc::Rc;
use std::cell::RefCell;

use crate::engine::sprite_sdl::Sprite;

struct Camera {
    has_changed: RefCell<bool>,
    position: RefCell<(f32, f32)>,
}

struct Entity<'a> {
    has_changed: RefCell<bool>,

    position: RefCell<(f32, f32)>,

    is_visible: RefCell<bool>,
    sprite: Option<Sprite<'a>>,
}

struct EntityManager<'a> {
    entities: Vec<Rc<Entity<'a>>>,
}