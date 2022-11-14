use crate::ecs::{
    components::renderable::Renderable,
    ecs::{ComponentType, Ecs},
    entities::entity::Entity,
    systems::system::System,
};
use crate::Game;
use actix::prelude::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct Renderer {
    ecs: Arc<Addr<Ecs>>,
    canvas: Rc<RefCell<WindowCanvas>>,
}

impl Clone for Renderer {
    fn clone(&self) -> Renderer {
        let ecs = self.ecs.clone();
        Renderer {
            ecs,
            canvas: self.canvas.clone(),
        }
    }
}

impl System for Renderer {
    fn update(&mut self, entities: Vec<Arc<Mutex<Entity>>>) {
        Self::render(entities, &self.canvas.clone());
    }

    fn get_component_types(&self) -> Vec<ComponentType> {
        vec![ComponentType::Renderable]
    }
}

impl Renderer {
    pub fn new(ecs: Arc<Addr<Ecs>>, canvas: Rc<RefCell<WindowCanvas>>) -> Renderer {
        Renderer { ecs, canvas }
    }

    fn render(entities: Vec<Arc<Mutex<Entity>>>, canvas: &Rc<RefCell<WindowCanvas>>) {
        let mut canvas_in = canvas.borrow_mut();
        let texture_creator = canvas_in.texture_creator();

        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .expect("Unable to create texture");
        // Create a red-green gradient
        texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..256 {
                    for x in 0..256 {
                        let offset = y * pitch + x * 3;
                        buffer[offset] = x as u8;
                        buffer[offset + 1] = y as u8;
                        buffer[offset + 2] = 0;
                    }
                }
            })
            .expect("Unable to lock texture");

        canvas_in.clear();

        for entity in entities {
            if let Some(g) = entity
                .lock()
                .unwrap()
                .get_component(&ComponentType::Renderable)
            {
                if let Some(n) = g.as_any().downcast_ref::<Renderable>() {
                    canvas_in
                        .copy_ex(
                            &texture,
                            None,
                            Some(Rect::new(n.position.x, n.position.y, 100, 100)),
                            n.rotation,
                            None,
                            false,
                            false,
                        )
                        .expect("Unable to copy texture");
                }
            }
        }

        canvas_in.present();
    }
}
