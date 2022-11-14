use crate::ecs::{
    ecs::{ComponentType, Ecs},
    entities::entity::Entity,
    systems::system::System,
};
use actix::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct User {}

impl System for User {
    fn update(&mut self, _entities: Vec<Arc<Mutex<Entity>>>) {}

    fn get_component_types(&self) -> Vec<ComponentType> {
        vec![ComponentType::User]
    }
}

impl User {
    pub fn new(_ecs: &Arc<Addr<Ecs>>) -> User {
        User {}
    }
}
