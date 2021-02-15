use ggez::{
    Context, GameResult, graphics,
    nalgebra::Point2,
    graphics::Rect
};

use crate::utils::Movement;

pub struct Bullet {
    sprite: graphics::Mesh,
    speed: f32,
    moving: Movement,
    hitbox: Rect,
    pos: (f32, f32),
    nb_pierce: i8,
    life: i32
}

impl Bullet {
    pub fn new(
        ctx: &mut Context,
        speed: f32,
        moving: Movement,
        hitbox: Rect,
        nb_pierce: i8,
        life: i32
    ) -> GameResult<Bullet> {

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
            nb_pierce: nb_pierce, 
            life: life
        };

        Ok(bullet)
    }

    pub fn update(&mut self) -> bool {
        match self.moving {
            Movement::Up => {
                self.pos.1 -= self.speed;
                self.hitbox.y -= self.speed;
            },
            Movement::Left => {
                self.pos.0 -= self.speed;
                self.hitbox.x -= self.speed;
            },
            Movement::Down => {
                self.pos.1 += self.speed;
                self.hitbox.y += self.speed;
            },
            Movement::Right => {
                self.pos.0 += self.speed;
                self.hitbox.x += self.speed;
            }
        }

        self.life -= 1;

        if self.life <= 0 { false }
        else { true }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, (Point2::new(self.pos.0, self.pos.1),))?;

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn set_life(&mut self, new_life: i32) {
        self.life = new_life;
    }

    pub fn hit_something(&mut self) {
        self.nb_pierce -= 1;
    }

    pub fn get_nb_pierce(&self) -> i8 {
        self.nb_pierce
    }
}

