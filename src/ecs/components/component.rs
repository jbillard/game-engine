use crate::ecs::ecs::ComponentType;
use std::any::Any;
use std::fmt::{Debug, Result};

pub trait Component: Send + Sync + AToAny + ComponentClone {
    fn get_type(&self) -> ComponentType;
}

pub trait AToAny: 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T: 'static> AToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Debug for dyn Component {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.get_type())
    }
}

pub trait ComponentClone {
    fn clone_box(&self) -> Box<dyn Component>;
}

impl<T> ComponentClone for T
where
    T: 'static + Component + Clone,
{
    fn clone_box(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.clone_box()
    }
}
