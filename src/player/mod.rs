use crate::prelude::*;

mod animation;
mod controller;
mod state;

pub use animation::*;
pub use controller::*;
pub use state::*;

pub struct Player {
    flip: bool,
}

impl Player {
    fn new() -> Self {
        Self { flip: false }
    }
}

pub fn player_health(world: &mut World) {
    let mut enemy_option = None;
    world
        .query::<(&Enemy, &Transform)>()
        .iter()
        .for_each(|(_, (d, t))| enemy_option = Some((*d, *t)));

    world
        .query_mut::<With<Player, (&mut Health, &Transform)>>()
        .into_iter()
        .for_each(|(_, (health, transform))| {
            if let Some((enemy_data, enemy_transform)) = enemy_option {
                if enemy_transform.intersection_with(transform) {
                    health.take_damage(enemy_data.0.damage);
                }
            }

            // Update hud player healthbar
            let mut hud = storage::get_mut::<Hud>();
            for child in hud.get_element_mut::<TextureElement>("healthbar").unwrap().children.iter_mut() {
                let bar = child.as_any_mut().downcast_mut::<RectangleElement>().unwrap();
                bar.set_width((health.hitpoints / 100.0) * bar.get_width());
            }
        });
}

pub fn spawn_player(world: &mut hecs::World, position: DVec2) {
    let mut animator = Animator::new("idling");
    animator.add_animation("running", Animation::from_dir("assets/sprites/final/player/run"));
    animator.add_animation("idling", Animation::from_dir("assets/sprites/final/player/idle"));
    animator.add_animation("jumping", Animation::from_dir("assets/sprites/final/player/jump"));
    animator.add_animation("falling", Animation::from_dir("assets/sprites/final/player/fall"));

    world.spawn((
        Player::new(),
        PlayerState::None,
        Health::default(),
        Transform::new().set_scale(DVec2::new(6.0, 7.0)).set_translation(position),
        KinematicBody::from_friction(DVec2::new(0.8, 1.0)),
        PlayerController::new(),
        animator,
    ));
}

pub fn player_system_bundle(world: &mut World) {
    player_input(world);
    player_controller(world);
    player_health(world);
    player_animations(world);
    update_player_states(world);
}
