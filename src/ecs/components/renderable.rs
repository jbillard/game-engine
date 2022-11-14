use crate::ecs::{components::component::Component, ecs::ComponentType};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Renderable {
    pub position: Point,
    pub rotation: f64,
}

impl Component for Renderable {
    fn get_type(&self) -> ComponentType {
        ComponentType::Renderable
    }
}
