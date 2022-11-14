use crate::ecs::{
    components::{
        renderable::{Point, Renderable},
        solid::Solid,
    },
    entities::entity::{BoxedComponent, Entity},
    systems::{
        physics::Physics,
        renderer::Renderer,
        system::{ActorSystem, GetComponetTypes, Update},
        user::User,
    },
};
use crate::Game;
use actix::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
pub struct Init;

#[derive(Message)]
#[rtype(result = "Option<Arc<WindowCanvas>>")]
pub struct GetCanvas;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ComponentType {
    User,
    Movable,
    Position,
    Renderable,
    Hostile,
    Damage,
    Solid,
    Health,
    Hud,
    Custom,
}

impl Display for ComponentType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub struct Ecs {
    pub entities: HashMap<Uuid, Arc<Mutex<Entity>>>,
    pub cached_entities: HashMap<String, Vec<Uuid>>,
    pub canvas: Option<Rc<RefCell<WindowCanvas>>>,
}

impl Actor for Ecs {
    type Context = Context<Self>;
}

impl Handler<Init> for Ecs {
    type Result = ResponseFuture<Result<(), ()>>;

    fn handle(&mut self, _msg: Init, ctx: &mut Context<Self>) -> Self::Result {
        let sdl_context = sdl2::init().expect("Unable to get context");
        let canvas = Rc::new(RefCell::new(Game::get_canvas(&sdl_context.clone())));
        self.canvas = Some(Rc::clone(&canvas));
        let addr = Arc::new(ctx.address());

        let actor_systems = vec![
            ActorSystem::new(Box::new(Renderer::new(
                Arc::clone(&addr),
                Rc::clone(&canvas),
            ))),
            ActorSystem::new(Box::new(User::new(&Arc::clone(&addr)))),
            ActorSystem::new(Box::new(Physics::new(Arc::clone(&addr)))),
        ];

        self.create_entity(vec![Box::new(Renderable {
            position: Point { x: 50, y: 60 },
            rotation: 3.,
        })]);

        self.create_entity(vec![Box::new(Solid {})]);

        self.create_entity(vec![
            Box::new(Renderable {
                position: Point { x: 100, y: 80 },
                rotation: 2.,
            }),
            Box::new(Solid {}),
        ]);

        let entities = self.entities.clone();
        let mut cached_entities = self.cached_entities.clone();
        Box::pin(async move {
            let mut system_addrs = vec![];

            for actor_system in actor_systems {
                system_addrs.push(actor_system.start());
            }

            let mut event_pump = sdl_context.event_pump().expect("Unable to get event pump");
            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }
                //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

                Self::update(&system_addrs, &entities, &mut cached_entities).await;
            }

            Ok(())
        })
    }
}

impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            entities: HashMap::new(),
            cached_entities: HashMap::new(),
            canvas: None,
        }
    }

    pub fn create_entity(&mut self, components: Vec<BoxedComponent>) {
        let mut components_mapped = HashMap::new();
        for component in components {
            components_mapped.insert(component.get_type(), component);
        }
        let entity = Entity::new(components_mapped);
        let uid = entity.uid;
        self.entities
            .entry(uid)
            .or_insert_with(|| Arc::new(Mutex::new(entity)));
    }

    async fn update(
        system_addrs: &[Addr<ActorSystem>],
        entities: &HashMap<Uuid, Arc<Mutex<Entity>>>,
        cached_entities: &mut HashMap<String, Vec<Uuid>>,
    ) {
        for system_addr in system_addrs {
            system_addr
                .send(Update {
                    entities: Self::get_entities_from(
                        entities,
                        cached_entities,
                        &system_addr.send(GetComponetTypes {}).await.unwrap(),
                    ),
                })
                .await
                .expect("Unable to send GetComponentTypes message");
        }
    }

    pub fn get_entities_from(
        entities: &HashMap<Uuid, Arc<Mutex<Entity>>>,
        cached_entities: &mut HashMap<String, Vec<Uuid>>,
        component_types: &[ComponentType],
    ) -> Vec<Arc<Mutex<Entity>>> {
        let mut component_types_text = Self::get_string_vec(&component_types);
        component_types_text.sort();
        let key = component_types_text.join("_");

        if !cached_entities.contains_key(&key) {
            let mut entity_uids: Vec<Uuid> = Vec::new();
            for shared_entity in entities.values() {
                let entity = shared_entity.lock().unwrap();
                for component in entity.get_components() {
                    if component_types.contains(&component.get_type()) {
                        entity_uids.push(entity.uid);
                    }
                }
            }

            cached_entities.entry(key.clone()).or_insert(entity_uids);
        }

        match cached_entities.get(&key) {
            Some(uids) => {
                let mut new_entities = vec![];
                for uid in uids {
                    if let Some(ent) = entities.get(uid) {
                        new_entities.push(Arc::clone(ent));
                    }
                }
                new_entities
            }
            _ => vec![],
        }
    }

    fn get_string_vec(component_types: &[ComponentType]) -> Vec<String> {
        component_types
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
    }
}
