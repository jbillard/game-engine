use crate::ecs::{
    components::renderable::Renderable,
    ecs::{ComponentType, Ecs},
    entities::entity::Entity,
    systems::system::System,
};
use actix::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Physics {
    ecs: Arc<Addr<Ecs>>,
}

impl System for Physics {
    fn update(&mut self, entities: Vec<Arc<Mutex<Entity>>>) {
        for entity in entities {
            if let Some(g) = entity
                .lock()
                .unwrap()
                .get_component(&ComponentType::Renderable)
            {
                if let Some(ref mut n) = g.as_mut_any().downcast_mut::<Renderable>() {
                    n.rotation += 1.;
                }
            }
        }
    }

    fn get_component_types(&self) -> Vec<ComponentType> {
        vec![ComponentType::Renderable]
    }
}

impl Physics {
    pub fn new(ecs: Arc<Addr<Ecs>>) -> Physics {
        Physics { ecs }
    }
}
