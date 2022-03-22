use crate::prelude::*;

pub struct Rad(pub f64);

pub struct Bird;

pub struct ReturnPoint(DVec2);

impl Default for ReturnPoint {
    fn default() -> Self {
        Self(DVec2::new(100.0, 100.0))
    }
}

pub enum BirdState {
    Lookout { find_radius: f64 },
    Chasing { lose_radius: f64 },
    Returning { point: ReturnPoint },
}

impl BirdState {
    fn lookout() -> Self {
        Self::Lookout { find_radius: 40.0 }
    }

    fn chasing() -> Self {
        Self::Chasing { lose_radius: 70.0 }
    }

    fn returning() -> Self {
        Self::Returning {
            point: ReturnPoint::default(),
        }
    }
}

pub fn bird_state_system(world: &mut World) {
    let mut player_pos_option = None;
    world
        .query::<With<Player, &Transform>>()
        .iter()
        .for_each(|(_, t)| player_pos_option = Some(t.center()));
    let player_pos = player_pos_option.unwrap_or_default();

    world
        .query_mut::<With<Bird, (&Transform, &mut BirdState)>>()
        .into_iter()
        .for_each(|(_, (transform, state))| {
            let distance = transform.center().distance(player_pos);

            match state {
                BirdState::Lookout { find_radius } => {
                    if distance < *find_radius {
                        *state = BirdState::chasing();
                    }
                }
                BirdState::Chasing { lose_radius } => {
                    if distance > *lose_radius {
                        *state = BirdState::returning()
                    }
                }
                BirdState::Returning { point } => {
                    if transform.center().distance(point.0).round().abs() <= 1.0 {
                        *state = BirdState::lookout()
                    }
                }
            }
        });
}

pub fn bird_ai_system(world: &mut World) {
    let mut player_pos_option = None;
    world
        .query::<(&Player, &Transform)>()
        .iter()
        .for_each(|(_, (_, t))| player_pos_option = Some(t.center()));
    let player_pos = player_pos_option.unwrap_or_default();

    world
        .query_mut::<With<Bird, (&Transform, &mut BirdState, &mut KinematicBody, &mut Rad, &mut f64)>>()
        .into_iter()
        .for_each(|(_, (transform, state, body, angle, val))| {
            *val += get_delta_time() * 2.0;

            fn goto(pos: DVec2, target: DVec2, body: &mut KinematicBody) -> f64 {
                let angle = f64::atan2(target.y - pos.y, target.x - pos.x);
                body.acceleration += DVec2::new(angle.cos(), angle.sin()).normalize_or_zero() * 10.0;
                angle
            }

            match state {
                BirdState::Lookout { .. } => {
                    body.velocity = DVec2::new(val.sin(), val.cos()) * 40.0;
                }
                BirdState::Chasing { .. } => {
                    angle.0 = goto(transform.center(), player_pos, body);
                }
                BirdState::Returning { point } => {
                    angle.0 = goto(transform.center(), point.0, body);
                }
            }
        });
}

pub fn render_bird_debug(world: &World, c: Context, gl: &mut GlGraphics) {
    use graphics::*;

    fn draw_debug(transform: &Transform, angle: &Rad, radius: &f64, color: [f32; 4], c: Context, gl: &mut GlGraphics) {
        let new_angle = DVec2::new(angle.0.sin(), angle.0.cos()).normalize_or_zero() * *radius;

        line(
            [0.0, 1.0, 0.0, 1.0],
            0.2,
            [
                transform.center().x,
                transform.center().y,
                transform.center().x + new_angle.y,
                transform.center().y + new_angle.x,
            ],
            c.transform,
            gl,
        );

        ellipse(
            color,
            [0.0, 0.0, *radius * 2.0, *radius * 2.0],
            c.transform.trans(transform.center().x - radius, transform.center().y - radius),
            gl,
        )
    }

    world
        .query::<With<Bird, (&BirdState, &Transform, &Rad)>>()
        .iter()
        .for_each(|(_, (state, transform, angle))| match state {
            BirdState::Lookout { find_radius } => draw_debug(transform, angle, &find_radius, [1.0, 0.0, 0.0, 0.15], c, gl),
            BirdState::Chasing { lose_radius } => draw_debug(transform, angle, &lose_radius, [0.0, 0.0, 1.0, 0.15], c, gl),
            _ => (),
        })
}

pub fn spawn_bird(world: &mut hecs::World) {
    let transform = Transform::new()
        .set_scale(DVec2::new(10.0, 6.0))
        .set_translation(DVec2::new(ReturnPoint::default().0.x, ReturnPoint::default().0.y));
    let mut animator = Animator::new("flying");
    animator.add_animation("flying", Animation::from_dir("assets/sprites/final/birdenemy/fly"));

    world.spawn((
        Bird,
        BirdState::lookout(),
        Enemy(EnemyData { damage: 25.0 }),
        transform,
        0.0_f64,
        Rad(0.0),
        KinematicBody {
            friction: DVec2::new(0.85, 0.85),
            has_mass: false,
            ..Default::default()
        },
        animator,
    ));
}
