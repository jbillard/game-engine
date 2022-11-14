use crate::ecs::{ecs::ComponentType, entities::entity::Entity};
use actix::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub entities: Vec<Arc<Mutex<Entity>>>,
}

pub struct GetComponetTypes;

impl Message for GetComponetTypes {
    type Result = Vec<ComponentType>;
}

pub trait System: SystemClone {
    fn update(&mut self, entities: Vec<Arc<Mutex<Entity>>>);
    fn get_component_types(&self) -> Vec<ComponentType>;
}

#[derive(Clone)]
pub struct ActorSystem {
    pub system: Box<dyn System>,
}

impl Actor for ActorSystem {
    type Context = Context<Self>;
}

impl ActorSystem {
    pub fn new(system: Box<dyn System>) -> ActorSystem {
        ActorSystem { system }
    }
}

impl Handler<Update> for ActorSystem {
    type Result = ();

    fn handle(&mut self, msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        self.system.update(msg.entities);
    }
}

impl Handler<GetComponetTypes> for ActorSystem {
    type Result = MessageResult<GetComponetTypes>;

    fn handle(&mut self, _msg: GetComponetTypes, _ctx: &mut Context<Self>) -> Self::Result {
        MessageResult(self.system.get_component_types())
    }
}

pub trait SystemClone {
    fn clone_box(&self) -> Box<dyn System>;
}

impl<T> SystemClone for T
where
    T: 'static + System + Clone,
{
    fn clone_box(&self) -> Box<dyn System> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn System> {
    fn clone(&self) -> Box<dyn System> {
        self.clone_box()
    }
}
