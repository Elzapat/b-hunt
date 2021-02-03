// use rand::Rng;
// use crate::particle::Particle;
// use ggez::{
//     Context, GameResult,
//     graphics,
//     graphics::{ Color }
// };
//
// struct ParticleSystem {
//     particles: Vec<Particle>,
//     spawn_rate: f32,
//     main_color: Color
// }
//
// impl ParticleSystem {
//     pub fn new(spawn_rate: f32, main_color: Color) -> ParticleSystem {
//         ParticleSystem {
//             particles: vec![],
//             spawn_rate: spawn_rate,
//             main_color: main_color
//         }
//     }
//
//     pub fn update(&mut self, fps: f64) {
//         for p in self.particles.iter() {
//             p.update(fps);
//         }
//     }
//
//     pub fn draw(&self, ctx: &mut Context) -> GameResult {
//         for p in self.particles.iter() {
//             p.draw(ctx)?;
//         }
//
//         Ok(())
//     }
// }
