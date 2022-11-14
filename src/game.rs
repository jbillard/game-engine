use crate::ecs::ecs::{Ecs, Init};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{render::WindowCanvas, Sdl};
use std::time::Duration;

use actix::Actor;

pub struct Game {
    ecs: Ecs,
}

impl Game {
    pub fn new() -> Game {
        Game { ecs: Ecs::new() }
    }

    pub async fn start(self) {
        if let Err(err) = self.ecs.start().send(Init {}).await {
            println!("{}", err);
        }
    }

    pub fn get_canvas(sdl_context: &Sdl) -> WindowCanvas {
        let video_subsystem = sdl_context.video().expect("Unable to get video subsytem");
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .expect("Unable to get window");

        window.into_canvas().build().expect("Unable to get canvas")
    }
}
