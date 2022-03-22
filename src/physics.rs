use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct Gravity(pub DVec2);

// #[derive(Default, Debug, Clone, Copy)]
// pub struct Velocity(pub DVec2);

// #[derive(Default, Debug, Clone, Copy)]
// pub struct Acceleration(pub DVec2);

// #[derive(Default, Debug, Clone, Copy)]
// pub struct Friction(pub DVec2);

pub struct StaticBody;

#[derive(Default)]
pub struct KinematicBody {
    pub velocity: DVec2,
    pub acceleration: DVec2,
    pub friction: DVec2,
    pub collisions: Collision,
    pub has_mass: bool,
}

#[allow(unused)]
/// Strongly adviced not to use the default implementation
impl KinematicBody {
    pub fn new() -> Self {
        Self {
            friction: DVec2::new(1.0, 1.0),
            has_mass: true,
            ..Default::default()
        }
    }

    pub fn from_friction(friction: DVec2) -> Self {
        Self {
            friction,
            has_mass: true,
            ..Default::default()
        }
    }

    pub fn apply_force(&mut self, force: DVec2) {
        self.acceleration += force
    }

    pub fn set_max_velocity(&mut self, max: DVec2) {
        self.velocity.clamp(DVec2::ZERO, max);
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Collision {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

///physics system handles velocity and gravitation. It's recommended to run before all the logic code.
fn physics(world: &mut hecs::World) {
    world
        .query::<(&mut Transform, &mut KinematicBody)>()
        .into_iter()
        .for_each(|(_, (transform, body))| {
            if body.has_mass {
                body.apply_force(storage::get::<Gravity>().0);
            }

            body.velocity += body.acceleration;
            body.velocity *= body.friction;
            *transform = transform.set_translation(transform.translation + body.velocity * get_delta_time());
            body.acceleration = DVec2::ZERO;
        });
}

pub fn render_debug_boxes_system(world: &mut World, c: Context, gl: &mut GlGraphics) {
    use graphics::Rectangle;

    // Render transforms
    world
        .query::<&Transform>()
        .iter()
        .for_each(|(_, transform)| Rectangle::new_border([0.15, 0.25, 1.0, 1.0], 0.2).draw(transform.into_array(), &c.draw_state, c.transform, gl));
}

/// AABB system is a simple aabb rectangle collision system and should run after the logic systems.
pub fn aabb(world: &mut World) {
    let mut statics = vec![];
    world.query::<With<StaticBody, &Transform>>().iter().for_each(|(_, t)| statics.push(*t));

    world
        .query_mut::<(&mut KinematicBody, &mut Transform)>()
        .into_iter()
        .for_each(|(_, (kinematic_body, transform))| {
            kinematic_body.collisions = Collision::default();
            for static_transform in statics.iter() {
                let distance = DVec2::new(
                    static_transform.center().x - transform.center().x,
                    static_transform.center().y - transform.center().y,
                );
                let intersect = DVec2::new(
                    distance.x.abs() - (static_transform.half_scale().x + transform.half_scale().x),
                    distance.y.abs() - (static_transform.half_scale().y + transform.half_scale().y),
                );

                let intersection;
                if intersect.x < 0.0 && intersect.y < 0.0 {
                    if intersect.x > intersect.y {
                        if distance.x > 0.0 {
                            kinematic_body.velocity.x = 0.0;
                            kinematic_body.collisions.right = true;
                            intersection = DVec2::new(intersect.x, 0.0);
                        } else {
                            kinematic_body.velocity.x = 0.0;
                            kinematic_body.collisions.left = true;
                            intersection = DVec2::new(-intersect.x, 0.0);
                        }
                    } else {
                        if distance.y > 0.0 {
                            kinematic_body.velocity.y = 0.0;
                            kinematic_body.collisions.bottom = true;
                            intersection = DVec2::new(0.0, intersect.y);
                        } else {
                            kinematic_body.velocity.y = 0.0;
                            kinematic_body.collisions.top = true;
                            intersection = DVec2::new(0.0, -intersect.y);
                        }
                    }

                    transform.translation += intersection;
                }
            }
        });
}

pub fn physics_system_bundle(world: &mut World) {
    physics(world);
    aabb(world)
}
