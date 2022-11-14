use crate::ecs::{components::component::Component, ecs::ComponentType};

#[derive(Clone)]
pub struct Solid {}

impl Component for Solid {
    fn get_type(&self) -> ComponentType {
        ComponentType::Solid
    }
}
