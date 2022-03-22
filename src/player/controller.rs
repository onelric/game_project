use crate::prelude::*;

#[derive(Default)]
struct PlayerInput {
    right: bool,
    left: bool,
    jump: bool,
}

#[derive(Default)]
pub struct PlayerController {
    jump_force: f64,
    movement_speed: f64,
    input: PlayerInput,
}

impl PlayerController {
    pub fn new() -> Self {
        Self {
            jump_force: 250.0,
            movement_speed: 25.0,
            ..Default::default()
        }
    }

    pub fn dir_vector(&self) -> DVec2 {
        let horizontal = bool_to_float::<f64>(self.input.right) - bool_to_float::<f64>(self.input.left);
        DVec2::new(horizontal, 0.0).normalize_or_zero()
    }
}

pub fn player_input(world: &mut World) {
    world.query::<&mut PlayerController>().into_iter().for_each(|(_, controller)| {
        let args = storage::get::<Option<ButtonArgs>>().to_owned();

        if let Some(buttons) = args {
            match buttons.state {
                piston::ButtonState::Press => match buttons.button {
                    Button::Keyboard(Key::A) => controller.input.left = true,
                    Button::Keyboard(Key::D) => controller.input.right = true,
                    Button::Keyboard(Key::Space) => controller.input.jump = true,
                    _ => (),
                },
                piston::ButtonState::Release => match buttons.button {
                    Button::Keyboard(Key::A) => controller.input.left = false,
                    Button::Keyboard(Key::D) => controller.input.right = false,
                    Button::Keyboard(Key::Space) => controller.input.jump = false,
                    _ => (),
                },
            }
        }
    })
}

pub fn player_controller(world: &mut World) {
    world
        .query::<(&mut Player, &mut PlayerController, &mut KinematicBody)>()
        .into_iter()
        .for_each(|(_, (player, controller, body))| {
            if body.velocity.x > 0.0 {
                player.flip = false
            } else if body.velocity.x < 0.0 {
                player.flip = true
            }

            if controller.input.jump && body.collisions.bottom {
                body.apply_force(DVec2::new(0.0, -controller.jump_force));
            }

            body.apply_force(DVec2::new(controller.dir_vector().x * controller.movement_speed, 0.0));
            body.set_max_velocity(DVec2::new(100.0, 100.0));
        });
}
