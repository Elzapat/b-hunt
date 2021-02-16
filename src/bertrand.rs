use ggez::{
    Context, GameResult, graphics,
    graphics::{
        Rect, DrawParam
    },
    nalgebra::Point2
};
use crate::map::Tree;
use std::collections::HashMap;

pub struct Bertrand {
    sprites: HashMap<String, graphics::Image>,
    facing: String,
    animation_cycle: u8,
    animation_frames: u32,
    swinging: (bool, u32),
    hitbox: Rect,
    speed: f32,
}

impl Bertrand {
    pub fn new(ctx: &mut Context, hitbox: Rect) -> GameResult<Bertrand> {
        let mut sprites = HashMap::new();

        for facing in ["front", "back", "right", "left"].iter() {
            for i in 0..=5 {
                let image = graphics::Image::new(ctx, format!("/Bertrand/Bertrand_{}_{}.png", facing, i))?;
                sprites.insert(format!("{}_{}", facing, i), image);
            }
        }

        let bertrand = Bertrand {
            sprites: sprites,
            facing: "front".to_string(),
            hitbox: hitbox,
            animation_cycle: 0,
            animation_frames: 0,
            swinging: (false, 0),
            speed: 1.0
        };

        Ok(bertrand)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.swinging.0 {
            if self.animation_frames < 10 { self.animation_cycle = 0; }
            else if self.animation_frames < 20 { self.animation_cycle = 1; }
            else if self.animation_frames < 30 { self.animation_cycle = 2; }
            else { self.animation_cycle = 3; }
        } else {
            if self.swinging.1 < 10 { self.animation_cycle = 4; }
            else { self.animation_cycle = 5; }
            self.swinging.1 += 1;
        }

        self.animation_frames = (self.animation_frames + 1) % 40;

        let sprite = self.sprites.get(&format!("{}_{}", self.facing, self.animation_cycle)).unwrap();
        let param = DrawParam::default()
            .dest(Point2::new(self.hitbox.x - 3.0, self.hitbox.y));
        graphics::draw(ctx, sprite, param)?;

        Ok(())
    }

    pub fn update(&mut self, fabien_pos: (f32, f32), trees: &mut Vec<Tree>) -> GameResult {
        if self.swinging.0 { return Ok(()) }

        if fabien_pos.0 < self.hitbox.x {
            self.hitbox.x -= self.speed;
            self.facing = "left".to_string();
        } else if fabien_pos.0 > self.hitbox.x {
            self.hitbox.x += self.speed;
            self.facing = "right".to_string();
        } else if fabien_pos.1 < self.hitbox.y {
            self.hitbox.y -= self.speed;
            self.facing = "back".to_string();
        } else if fabien_pos.1 > self.hitbox.y {
            self.hitbox.y += self.speed;
            self.facing = "front".to_string();
        } 

        if self.hitbox.x < fabien_pos.0 + self.speed && self.hitbox.x > fabien_pos.0 - self.speed {
            self.hitbox.x = fabien_pos.0;
        }
        if self.hitbox.y < fabien_pos.1 + self.speed && self.hitbox.y > fabien_pos.1 - self.speed {
            self.hitbox.y = fabien_pos.1;
        }

        for tree in trees.iter() {
            if self.hitbox.overlaps(&tree.get_hitbox()) {
                match &self.facing[..] {
                    "right" => {
                        self.hitbox.x -= self.speed;
                        self.hitbox.y += self.speed;
                        self.facing = "front".to_string();
                    },
                    "left" => {
                        self.hitbox.x += self.speed;
                        self.hitbox.y += self.speed;
                        self.facing = "front".to_string();
                    },
                    "back" => {
                        self.hitbox.y += self.speed;
                        self.hitbox.x += self.speed;
                        self.facing = "right".to_string();
                    },
                    "front" => {
                        self.hitbox.y -= self.speed;
                        self.hitbox.x += self.speed;
                        self.facing = "right".to_string();
                    },
                    _ => unreachable!()
                } 
            }
        }

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn is_dead(&self) -> bool {
        self.swinging.1 > 20
    }

    pub fn is_swinging(&self) -> bool {
        self.swinging.0
    }

    pub fn swing(&mut self) {
        self.swinging.0 = true;
    }
}
