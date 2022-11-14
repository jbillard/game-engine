use crate::ecs::{components::component::Component, ecs::ComponentType};
use std::collections::HashMap;
use uuid::Uuid;

pub type BoxedComponent = Box<dyn Component>;

#[derive(Debug, Clone)]
pub struct Entity {
    pub uid: Uuid,
    pub components: HashMap<ComponentType, BoxedComponent>,
}

impl Entity {
    pub fn new(components: HashMap<ComponentType, BoxedComponent>) -> Entity {
        Entity {
            uid: Uuid::new_v4(),
            components,
        }
    }
    pub fn add_component(&mut self, component: BoxedComponent) {
        self.components
            .entry(component.get_type())
            .or_insert(component);
    }

    pub fn get_components(&self) -> Vec<&BoxedComponent> {
        self.components.values().collect::<Vec<&BoxedComponent>>()
    }

    pub fn get_component(&mut self, component_type: &ComponentType) -> Option<&mut BoxedComponent> {
        self.components.get_mut(component_type)
    }

    pub fn get_component2<T: 'static>(&mut self, component_type: &ComponentType) -> Option<&mut T> {
        match self.components.get_mut(component_type) {
            Some(g) => g.as_mut_any().downcast_mut::<T>(),
            None => None,
        }
    }

    pub fn get_mut_component(
        &mut self,
        component_type: &ComponentType,
    ) -> Option<&mut BoxedComponent> {
        self.components.get_mut(component_type)
    }
}
