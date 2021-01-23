use ggez::{
    Context, GameResult, graphics,
    graphics::{
        Rect, DrawParam
    },
    nalgebra::Point2
};

pub struct Bertrand {
    //sprite: HashMap<String, graphics::Image>,
    //sprite: graphics::Mesh,
    sprite: graphics::Image,
    facing: String,
    hitbox: Rect,
    animation_cycle: u8,
    speed: f32,
}

impl Bertrand {
    pub fn new(ctx: &mut Context, hitbox: Rect) -> GameResult<Bertrand> {
        let sprite = graphics::Image::new(ctx, "/Fabien_right_1.png")?;

        let bertrand = Bertrand {
            sprite: sprite,
            facing: "front".to_string(),
            hitbox: hitbox,
            animation_cycle: 0,
            speed: 0.5,
        };

        Ok(bertrand)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::default().dest(Point2::new(self.hitbox.x, self.hitbox.y)))?;

        Ok(())
    }

    pub fn update(&mut self, fabien_pos: (f32, f32)) -> GameResult {
        if fabien_pos.0 < self.hitbox.x {
            self.hitbox.x -= self.speed;
        } else if fabien_pos.0 > self.hitbox.x {
            self.hitbox.x += self.speed;
        }

        if fabien_pos.1 < self.hitbox.y {
            self.hitbox.y -= self.speed;
        } else if fabien_pos.1 > self.hitbox.y {
            self.hitbox.y += self.speed;
        }

        if self.hitbox.x < fabien_pos.0 + self.speed && self.hitbox.x > fabien_pos.0 - self.speed {
            self.hitbox.x = fabien_pos.0;
        }
        if self.hitbox.y < fabien_pos.1 + self.speed && self.hitbox.y > fabien_pos.1 - self.speed {
            self.hitbox.y = fabien_pos.1;
        }

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }
}
