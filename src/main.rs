extern crate serde;
extern crate serde_json;

extern crate glam;
extern crate hecs;
extern crate num;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod animator;
mod ecs;
mod entity;
mod eventhandler;
mod game;
mod gamestate;
mod physics;
mod player;
mod prelude;
mod states;
mod utilities;
mod window;

// Alisa and Alex were here <3 :))
fn main() {
    use {
        crate::prelude::{storage, EventHandler, GameState},
        core::config::get_config,
    };

    storage::store(get_config());

    EventHandler::new::<GameState>().run()
}
