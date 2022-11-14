#![warn(clippy::all, clippy::pedantic)]
#![windows_subsystem = "windows"]

mod ecs;
mod game;

use game::Game;

#[actix::main]
async fn main() {
    Game::new().start().await;
}
