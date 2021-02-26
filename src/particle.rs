use ggez::{
    graphics, GameResult, Context,
    graphics::{ Color, Rect },
    nalgebra::{ Point2, Vector2 }
};

pub struct Particle {
    sprite: graphics::Mesh,
    position: Point2<f32>,
    speed: f32,
    rotation_speed: f32,
    rotation: f32,
    angle: f32,
    life: (f32, f32)
}

impl Particle {
    pub fn new(
        pos: Point2<f32>,
        speed: f32,
        rotation_speed: f32,
        angle: f32,
        life: f32,
        color: Color,
        size: f32,
        ctx: &mut Context
    ) -> GameResult<Particle> {
    
        let sprite = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, size, size),
            color
        )?;

        let particle = Particle {
            sprite: sprite,
            position: pos,
            speed: speed,
            rotation_speed: rotation_speed,
            rotation: 0.0,
            angle: angle,
            life: (life, life) 
        };

        Ok(particle)
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        let vel_x = dt * self.speed * self.angle.cos();
        let vel_y = dt * self.speed * self.angle.sin();
        
        self.position.x += vel_x;
        self.position.y += vel_y;
        self.rotation += dt * self.rotation_speed;

        self.life.1 -= ggez::timer::delta(ctx).as_secs_f32();
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let scale = self.life.1 / self.life.0; 
        let param = graphics::DrawParam::default()
            .dest(self.position)
            .rotation(self.rotation)
            .scale(Vector2::new(scale, scale));

        graphics::draw(ctx, &self.sprite, param)?;
        Ok(())
    }

    pub fn is_dead(&self) -> bool {
        self.life.1 <= 0.0
    }
}
