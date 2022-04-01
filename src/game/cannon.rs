use crate::prelude::*;

pub struct Cannon;

pub fn spawn_cannon(world: &mut World) {
    world.spawn((
        Transform::new()
            .set_scale(DVec2::new(10.0, 4.0))
            .set_translation(DVec2::new(200.0, 100.0)),
        Cannon,
    ));
}

pub fn cannon_update(world: &mut World) {
    let mut cannon_option = None;
    world.query::<With<Cannon, &Transform>>().iter().for_each(|(_, t)| {
        cannon_option = Some(*t);
    });

    world.query_mut::<With<Player, &mut Transform>>().into_iter().for_each(|(_, transform)| {
        if let Some(mut cannon_transform) = cannon_option {
            if cannon_transform.intersection_with(transform) {
                cannon_transform.translation = transform.translation;
            }
        }
    });

    world.query_mut::<With<Cannon, &mut Transform>>().into_iter().for_each(|(_, tf)| {
        if let Some(transform) = cannon_option {
            *tf = transform;
            println!("{}", tf.translation);
        }
    });
}

pub fn cannon_systems(world: &mut World) {
    cannon_update(world);
}
