use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::nalgebra::Point2;

mod utils: use utils::*;

struct Bullet {
    sprite: graphics::Mesh,
    speed: f32,
    moving: Movement,
    hitbox: Rect,
    map_size: (f32, f32)
}

impl Bullet {
    pub fn new(ctx: &mut context,
               speed: f32,
               moving: Movement,
               hitbox: Rect,
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
            map_size: map_size
        }

        Ok(bullet)
    }

    pub fn update(&mut self) -> bool {
        match self.moving {
            Movement::Up => self.hitbox.y -= self.speed,
            Movement::Left => self.hitbox.x -= self.speed,
            Movement::Down => self.hitbox.y += self.speed,
            Movement::Rigt => self.hitbox.x += self.speed
        }

        if self.hitbox.x <= 0 ||
           self.hitbox.x >= self.map_size.0 ||
           self.hitbox.y <= 0 ||
           self.hitbox.y >= self.map_size.1 { false }
        else { true }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, self.sprite, (Point2::new(self.hitbox.x, self.hitbox.y, )))?;
    }
}

