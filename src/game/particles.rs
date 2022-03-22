// #![allow(unused)]

// use legion::world::SubWorld;

// use crate::prelude::*;

// #[derive(Clone)]
// pub enum ParticleShape {
//     Circle { radius: f64 },
//     Rectangle { width: f64, height: f64 },
// }

// #[derive(Clone)]
// pub enum EmissionShape {
//     Point,
//     Rectangle { width: f64, height: f64 },
// }

// #[derive(Clone)]
// pub struct Particle {
//     shape: ParticleShape,
//     emission_shape: EmissionShape,
//     velocity: Velocity,
//     lifetime: f64,
//     dead: bool,
// }

// impl Particle {
//     pub fn new() -> Self {
//         Self {
//             shape: ParticleShape::Rectangle {
//                 width: 1.0,
//                 height: 1.0,
//             },
//             emission_shape: EmissionShape::Point,
//             velocity: Velocity::default(),
//             lifetime: 1.0,
//             dead: false,
//         }
//     }
// }

// pub struct ParticleEmitter {
//     particles: Vec<Particle>,
//     particle_base: Particle,
//     emit_point: DVec2,
// }

// impl ParticleEmitter {
//     pub fn new(particle_base: Particle) -> Self {
//         Self {
//             particles: vec![],
//             particle_base,
//             emit_point: DVec2::ZERO,
//         }
//     }

//     pub fn set_particle_base(mut self, particle_base: Particle) -> Self {
//         self.particle_base = particle_base;
//         self
//     }

//     pub fn emit(&mut self, position: DVec2) {
//         self.emit_point = position;
//         self.particles.push(Particle {
//             shape: self.particle_base.shape.clone(),
//             emission_shape: self.particle_base.emission_shape.clone(),
//             velocity: Velocity::default(),
//             lifetime: self.particle_base.lifetime,
//             dead: self.particle_base.dead,
//         })
//     }

//     pub fn apply_velocity(&mut self, velocity_range: std::ops::Range<DVec2>) {
//         let mut thread = thread_rng();

//         for particle in self.particles.iter_mut() {
//             particle.velocity = Velocity(DVec2::new(
//                 thread.gen_range(velocity_range.start.x..velocity_range.end.x),
//                 thread.gen_range(velocity_range.start.y..velocity_range.end.y),
//             ));
//             // self.emit_point += particle.velocity.0;
//         }
//     }

//     pub fn render_particles(&mut self, c: Context, gl: &mut GlGraphics) {
//         use graphics::*;

//         for particle in self.particles.iter() {
//             match particle.emission_shape {
//                 EmissionShape::Point => match particle.shape {
//                     ParticleShape::Circle { radius } => {}
//                     ParticleShape::Rectangle { width, height } => {
//                         let rect = rectangle::centered([
//                             self.emit_point.x + width / 2.0,
//                             self.emit_point.y + height / 2.0,
//                             width / 2.0,
//                             height / 2.0,
//                         ]);

//                         let rectangle = Rectangle::new_border([0.0, 0.0, 1.0, 1.0], 0.5);
//                         rectangle.draw(rect, &c.draw_state, c.transform, gl)
//                     }
//                 },
//                 EmissionShape::Rectangle { width, height } => (),
//             }
//         }
//     }
// }

// #[system]
// #[write_component(ParticleEmitter)]
// pub fn update_particles(world: &mut SubWorld, #[resource] dt: &DeltaTime) {
//     <&mut ParticleEmitter>::query()
//         .iter_mut(world)
//         .for_each(|emitter| {
//             emitter.particles = emitter.particles.drain(..).filter(|p| !p.dead).collect();
//             // println!("{}", emitter.particles.len());
//             emitter.particles.iter_mut().for_each(|particle| {
//                 particle.lifetime -= dt.0;
//                 if particle.lifetime <= 0.0 {
//                     particle.dead = true
//                 }
//                 emitter.emit_point += particle.velocity.0 * dt.0
//             });
//         })
// }

// pub fn render_particles_system(world: &mut World, c: Context, gl: &mut GlGraphics) {
//     <&mut ParticleEmitter>::query()
//         .iter_mut(world)
//         .for_each(|emitter| emitter.render_particles(c, gl));
// }
