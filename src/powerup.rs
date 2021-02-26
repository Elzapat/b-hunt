use ggez::{
    Context, graphics, GameResult,
    graphics::Rect,
    nalgebra::{ Vector2, Point2 }
};
use rand::Rng;
use crate::particle::Particle;

// Powerups
#[derive(Clone)]
pub enum Powerups {
    PiercingBullet((f32, u8)),
    SpeedBoost((f32, f32)),
    Heal(u8),
    AmmoRestock(u32)
}

pub struct Powerup {
    powerup: Powerups,
    sprite: graphics::Image,
    hitbox: Rect,
    scale: f32,
    particles: Vec<Particle>,
    time_since_last_particle: f64
}

impl Powerup {
    pub fn new(ctx: &mut Context, map_size: (f32, f32)) -> GameResult<Powerup> {
        let mut rng = rand::thread_rng();

        let new_powerup = match rng.gen_range(0..100) {
            0..=19 => Powerups::PiercingBullet((rng.gen_range(10..=20) as f32, rng.gen_range(1..=3))),
            20..=49 => Powerups::SpeedBoost((rng.gen_range(15..=20) as f32, rng.gen_range(14..17) as f32 / 10.0)),
            50..=69 => Powerups::Heal(rng.gen_range(1..=3)),
            _ => Powerups::AmmoRestock(rng.gen_range(15..=20))
        };

        let sprite_path = match new_powerup {
            Powerups::PiercingBullet(_) => "/piercing_bullet.png",
            Powerups::SpeedBoost(_) => "/speed_powerup.png",
            Powerups::Heal(_) => "/sandwich.png",
            Powerups::AmmoRestock(_) => "/bullet.png"
        };
        
        let sprite = graphics::Image::new(ctx, sprite_path)?;
        let scale = match new_powerup {
            Powerups::Heal(_) => 0.04,
            _ => 0.6
        };

        let hitbox = Rect::new(
            rng.gen_range(0..map_size.0 as u32) as f32,
            rng.gen_range(0..map_size.1 as u32) as f32,
            sprite.width() as f32 * scale,
            sprite.height() as f32 * scale
         );

        let powerup = Powerup {
            powerup: new_powerup,
            sprite: sprite,
            hitbox: hitbox,
            scale: scale,
            particles: vec![],
            time_since_last_particle: 0.0
        };

        Ok(powerup)
    }

    pub fn update(&mut self, ctx: &mut Context, time_passed: f64, fps: f64) -> GameResult {
        const SPEED: f32 = 3.0;
        const HEIGHT: f32 = 0.1;
        self.hitbox.y += (time_passed as f32 * SPEED).sin() * HEIGHT;

        self.time_since_last_particle += 1.0 / fps;

        const PARTICLE_SPAWN_RATE: f64 = 5.0;
        if self.time_since_last_particle > 1.0 / PARTICLE_SPAWN_RATE {
            self.time_since_last_particle = 0.0;

            let mut rng = rand::thread_rng();
            let r = 210 + (rng.gen_range(0..=80) - 40);
            let g = 210 + (rng.gen_range(0..=80) - 40);
            let b = 10 +  (rng.gen_range(0..=20) - 10);
            let color = graphics::Color::from_rgb(r, g, b);
            let pos = Point2::new(self.hitbox.x + self.hitbox.w / 2.0,
                self.hitbox.y + self.hitbox.h / 2.0);
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let size = rng.gen::<f32>() + 1.0; 
            let life = rng.gen::<f32>() * 2.0 + 1.0;
            const SPEED: f32 = 5.0;
            let coinflip = rand::random::<bool>();
            let rot_dir = if coinflip { -1.0 } else { 1.0 };
            let rot_speed = rot_dir * 3.0;

            self.particles.push(
                Particle::new(pos, SPEED, rot_speed, angle, life, color, size, ctx)?
            );
        }

        for p in self.particles.iter_mut() { p.update(ctx); }
        self.particles.retain(|p| !p.is_dead());

        Ok(())
    }
    
    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let param = graphics::DrawParam::default()
            .dest(Point2::new(self.hitbox.x, self.hitbox.y))
            .scale(Vector2::new(self.scale, self.scale));

        graphics::draw(ctx, &self.sprite, param)?;
        for p in self.particles.iter() { p.draw(ctx)?; }

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn get_powerup(&self) -> Powerups {
        self.powerup.clone()
    }

    pub fn get_sprite(&self) -> &graphics::Image {
        &self.sprite
    }
}
