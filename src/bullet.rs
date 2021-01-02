use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::graphics::Rect;

use crate::utils::Movement;

pub struct Bullet {
    sprite: graphics::Mesh,
    speed: f32,
    moving: Movement,
    hitbox: Rect,
    pos: (f32, f32),
    map_size: (f32, f32),
    life: u32
}

impl Bullet {
    pub fn new(ctx: &mut Context,
               speed: f32,
               moving: Movement,
               hitbox: Rect,
               life: u32,
               map_size: (f32, f32)) -> GameResult<Bullet> {

        let sprite = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            hitbox,
            graphics::Color::from_rgb(0, 0, 0)
        )?;

        let bullet = Bullet {
            sprite: sprite,
            speed: speed,
            moving: moving,
            hitbox: hitbox,
            pos: (0.0, 0.0),
            map_size: map_size,
            life: life
        };

        Ok(bullet)
    }

    pub fn update(&mut self) -> bool {
        match self.moving {
            Movement::Up => self.pos.1 -= self.speed,
            Movement::Left => self.pos.0 -= self.speed,
            Movement::Down => self.pos.1 += self.speed,
            Movement::Right => self.pos.0 += self.speed
        }

        self.life -= 1;

        if self.life <= 0 { false }
        else { true }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, (Point2::new(self.pos.0, self.pos.1),))?;

        Ok(())
    }
}

