pub mod entity;
pub mod entity_spawner;

use crate::log;

use self::entity::Entity;
use self::entity_spawner::EntitySpawner;

use std::{
    rc::{Rc}, cell::{RefCell}, collections::{HashMap, VecDeque}, error::{Error}
};

pub type EntityId = u32;

struct EntityManager<'a> {
    next_id: RefCell<EntityId>,
    available_ids: RefCell<VecDeque<EntityId>>,
    entities: RefCell<HashMap<EntityId, Rc<Entity<'a>>>>,
}

impl<'a> EntityManager<'a> {
    pub fn new() -> Self {
        EntityManager{
            next_id: 1.into(),
            available_ids: Default::default(),
            entities: Default::default(),
        }
    }

    pub fn remove_by_id(&self, id: EntityId) -> bool {
        match self.entities.borrow_mut().remove(&id) {
            Some(e) => true,
            None => false
        }
    }

    pub fn add(&self, entity: Entity<'a>) {
        let id = self.take_available_id();
        self.entities.borrow_mut().insert(id, entity.into());
    }

    pub fn add_multiple(&self, num: u32, entity_spawner: &'a EntitySpawner<'a>) {
        for _ in 0..num {
            self.add(entity_spawner.spawn());
        }
    }

    pub fn map_all_entities<
        F: Fn(EntityId, Rc<Entity<'a>>) -> Result<(), Box<dyn Error>>,
    >(&self, func: &F) {
        let entities = self.entities.take();

        for (id, entity) in entities.iter() {
            match func(*id, entity.clone()) {
                Err(e) => log!("Error: {:?}", e),
                Ok(_) => ()
            };
        }
        self.entities.replace(entities);
    }

    fn take_available_id(&self) -> EntityId {
        match self.available_ids.borrow_mut().pop_front() {
            Some(id) => id,
            None => self.next_id.replace_with(|id| { *id + 1 } )
        }
    }
}