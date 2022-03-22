use crate::prelude::{DVec2, Health, KinematicBody, With, World};

use super::Player;

#[derive(Debug, PartialEq, Eq)]
pub enum PlayerState {
    None,
    Moving,
    Jumping,
    Falling,
    Dead,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::None
    }
}

pub fn update_player_states(world: &mut World) {
    let query = world.query_mut::<With<Player, (&Health, &KinematicBody, &mut PlayerState)>>();

    for (_, (health, body, state)) in query {
        if body.velocity.abs().round() != DVec2::ZERO {
            if body.velocity.x.abs() > 0.0 {
                *state = PlayerState::Moving;
            }

            if body.velocity.y < 0.0 {
                *state = PlayerState::Jumping;
            } else if body.velocity.y > 0.0 {
                *state = PlayerState::Falling;
            }
        } else {
            *state = PlayerState::None;
        }

        if health.dead() {
            *state = PlayerState::Dead;
        }

        println!("{:#?}", state);
    }
}
