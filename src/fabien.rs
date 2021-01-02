use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::graphics::{Rect};
use ggez::{Context, GameResult};
use ggez::event;
use ggez::event::KeyCode;
use std::collections::{HashMap, VecDeque};

mod utils; use utils::Movement;
mod bullet; use bullet::*;

// Fabien is the player
pub struct Fabien {
    sprites: HashMap<String, graphics::Image>,
    facing: String,
    hitbox: Rect,
    shooting: (bool, u32),
    ammos: u32,
    score: u32,
    health: u8,
    animation_cycle: u8,
    speed: f32,
    movement_queue: VecDeque<Movement>,
    map_size: (f32, f32),
    shots: Vec<Bullet>
}

impl Fabien {
    pub fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<Fabien> {
        let hitbox = Rect::new(width / 2.0, height / 2.0, 8.0, 16.0);
        let mut sprites = HashMap::new();

        for _ in 0..4 {
            for facing in ["front", "back", "right", "left"].iter() {
                for i in 0..=4 {
                    let image = graphics::Image::new(ctx, format!("/Fabien_{}_{}.png", facing, i))?;
                    sprites.insert(format!("{}_{}", facing, i), image);
                }
            }
        }

        let fabien = Fabien {
            sprites: sprites,
            facing: "front".to_string(),
            hitbox: hitbox,
            shooting: (false, 0),
            ammos: 50,
            score: 0,
            health: 5,
            animation_cycle: 0,
            speed: 0.8,
            movement_queue: VecDeque::new(),
            map_size: (width, height),
            shots: Vec<Bullet>::new()
        };

        Ok(fabien)
    }

    pub fn draw(&mut self, ctx: &mut Context, frames: &u32) -> GameResult {
        if self.movement_queue.len() > 0 && !self.shooting.0 {
            match self.movement_queue.back().unwrap() {
                Movement::Up => self.facing = "back".to_string(),
                Movement::Left => self.facing = "left".to_string(),
                Movement::Down => self.facing = "front".to_string(),
                Movement::Right => self.facing = "right".to_string()
            }

            if *frames < 10 {
                self.animation_cycle = 0;
            } else if *frames < 20 {
                self.animation_cycle = 1;
            } else if *frames < 30 {
                self.animation_cycle = 2;
            } else { self.animation_cycle = 3; }
        } else if self.shooting.0 { self.animation_cycle = 4; }
        else { self.animation_ cycle = 0; }

        if self.shooting.0 { self.shots.iter().map(|b| b.draw()); }

        let sprite = self.sprites.get(&format!("{}_{}", self.facing, self.animation_cycle)).unwrap();
        graphics::draw(ctx, sprite, (Point2::new(self.hitbox.x, self.hitbox.y),))?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.movement_queue.len() > 0 && !self.shooting.0 {
            match self.movement_queue.back() {
                Some(Movement::Up) => {
                    if self.hitbox.y - self.speed > 0.0 {
                        self.hitbox.y -= self.speed;
                    }
                },
                Some(Movement::Left) => {
                    if self.hitbox.x - self.speed > 0.0 {
                        self.hitbox.x -= self.speed;
                    }
                },
                Some(Movement::Down) => {
                    if self.hitbox.y + self.hitbox.h + self.speed < self.map_size.1 {
                        self.hitbox.y += self.speed;
                    }
                },
                Some(Movement::Right) => {
                    if self.hitbox.x + self.hitbox.w + self.speed < self.map_size.0 {
                        self.hitbox.x += self.speed;
                    }
                },
                None => {}
            }
        }

        const CAMERA_SIZE: (f32, f32) = (200.0, 150.0);
        let mut camera_x = self.hitbox.x - CAMERA_SIZE.0 / 2.0;
        let mut camera_y = self.hitbox.y - CAMERA_SIZE.1 / 2.0;

        if camera_x <= 0.0 { camera_x = 0.0; }
        else if camera_x >= self.map_size.0 - CAMERA_SIZE.0 {
            camera_x = self.map_size.0 - CAMERA_SIZE.0;
        }

        if camera_y <= 0.0 { camera_y = 0.0 }
        else if camera_y >= self.map_size.1 - CAMERA_SIZE.1 {
            camera_y = self.map_size.1 -  CAMERA_SIZE.1;
        }

        graphics::set_screen_coordinates(ctx, Rect::new(
            camera_x,
            camera_y,
            CAMERA_SIZE.0,
            CAMERA_SIZE.1
        ))?;

        if self.shooting.0 { 
            self.shooting.1 += 1;
            self.shots.iter().map(|b| b.update());
        }
        if self.shooting.1 > 25 { self.shooting.0 = false; self.shooting.1 = 0; }

        Ok(())
    }

    pub fn key_down_event(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Z => self.movement_queue.push_back(Movement::Up),
            KeyCode::Q => self.movement_queue.push_back(Movement::Left),
            KeyCode::S => self.movement_queue.push_back(Movement::Down),
            KeyCode::D => self.movement_queue.push_back(Movement::Right),
            KeyCode::Space => {
                if !self.shooting.0 { 
                    self.shooting.0 = true;
                    self.ammos -= 1;

                    const BULLET_SPEED: f32 = 3.0;
                    let (mov, pos) = match self.facing {
                        "front".to_string() => (Movement::Down, (self.hitbox.x + 2.0, self.hitbox.y + 8.0))
                        "back".to_string() => (Movement::Up, (self.hitbox. + 7.0, self.hitbox.y + 9.0))
                        "left".to_string() => (Movement::Left, (self.hitbox.x, self.hitbox.y + 8.0))
                        "right".to_string() => (Movement::Right, (self.hitbox.x + 11.0, self.hitbox.y + 8.0))
                        _ => {}
                    };

                    let bullet = Bullet::new(
                        ctx,
                        BULLET_SPEED,
                        mov,
                        Rect::new(pos.0, pos.1, 3.0, 2.0),
                        self.map_size
                    );

                    self.shots.push(bullet);
                }
            },
            _ => {}
        }
    }

    pub fn key_up_event(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Z => self.movement_queue.retain(|mov| *mov != Movement::Up),
            KeyCode::Q => self.movement_queue.retain(|mov| *mov != Movement::Left),
            KeyCode::S => self.movement_queue.retain(|mov| *mov != Movement::Down),
            KeyCode::D => self.movement_queue.retain(|mov| *mov != Movement::Right),
            _ => {}
        }
    }
}
