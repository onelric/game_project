use hecs::World;

use crate::prelude::get_delta_time;

pub struct Health {
    pub hit: bool,
    pub hitpoints: f64,
    pub iframes: f64,
    iframes_timer: f64,
    received_damage: f64,
}

impl Default for Health {
    fn default() -> Self {
        let iframes = 0.5;
        Self {
            hit: Default::default(),
            received_damage: Default::default(),
            hitpoints: 100.0,
            iframes_timer: 0.0,
            iframes,
        }
    }
}

impl Health {
    pub fn take_damage(&mut self, amount: f64) {
        self.received_damage = amount;
        self.hit = true
    }

    pub fn dead(&self) -> bool {
        self.hitpoints <= 0.0
    }
}

impl std::fmt::Debug for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Health")
            .field("hit", &self.hit)
            .field("hitpoints", &self.hitpoints)
            .field("iframes", &self.iframes_timer)
            .field("received_damage", &self.received_damage)
            .finish()
    }
}

pub fn health_system(world: &mut World) {
    world.query_mut::<&mut Health>().into_iter().for_each(|(_, health)| {
        if health.hit {
            if health.iframes_timer == health.iframes {
                health.hitpoints -= health.received_damage;
            }
            health.iframes_timer -= get_delta_time();
        }
        if health.iframes_timer <= 0.0 {
            health.iframes_timer = health.iframes;
            health.hit = false
        }

        health.hitpoints = health.hitpoints.clamp(0.0, f64::MAX);
    })
}
