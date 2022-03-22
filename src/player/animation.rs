use crate::prelude::{Animator, World};

use super::{Player, PlayerState};

pub fn player_animations(world: &mut World) {
    world
        .query_mut::<(&Player, &PlayerState, &mut Animator)>()
        .into_iter()
        .for_each(|(_, (player, state, animator))| {
            animator.set_flip(player.flip);
            match state {
                PlayerState::None => animator.change_to("idling"),
                PlayerState::Moving => animator.change_to("running"),
                PlayerState::Jumping => animator.change_to("jumping"),
                PlayerState::Falling => animator.change_to("falling"),
                PlayerState::Dead => (),
            };
        })
}
