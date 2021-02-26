use ggez::{
    Context, GameResult, graphics,
    nalgebra::Point2,
    graphics::Rect
};

pub struct Bullet {
    sprite: graphics::Mesh,
    speed: f32,
    angle: f32,
    hitbox: Rect,
    pos: (f32, f32),
    nb_pierce: i8,
    life: f32
}

impl Bullet {
    pub fn new(
        ctx: &mut Context,
        speed: f32,
        angle: f32,
        hitbox: Rect,
        nb_pierce: i8,
        life: f32
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
            angle: angle,
            hitbox: hitbox,
            pos: (0.0, 0.0),
            nb_pierce: nb_pierce, 
            life: life
        };

        Ok(bullet)
    }

    pub fn update(&mut self, ctx: &mut Context) -> bool {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        let vel_x = dt * self.speed * self.angle.cos();
        let vel_y = dt * self.speed * self.angle.sin();

        self.hitbox.x += vel_x;
        self.hitbox.y += vel_y;
        self.pos.0 += vel_x;
        self.pos.1 += vel_y;

        self.life -= dt;

        !(self.life <= 0.0)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, (Point2::new(self.pos.0, self.pos.1),))?;

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn set_life(&mut self, new_life: f32) {
        self.life = new_life;
    }

    pub fn hit_something(&mut self) {
        self.nb_pierce -= 1;
    }

    pub fn get_nb_pierce(&self) -> i8 {
        self.nb_pierce
    }
}

