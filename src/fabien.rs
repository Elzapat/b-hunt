use std::collections::{ HashMap, VecDeque };

use ggez::{
    graphics, Context, GameResult,
    nalgebra::{ Point2, Vector2 },
    graphics::Rect,
    event::KeyCode
};

use crate::utils::Movement;
use crate::bullet::Bullet;

// Fabien is the player
pub struct Fabien {
    sprites: HashMap<String, graphics::Image>,
    bullet_sprite: graphics::Image,
    facing: String,
    hitbox: Rect,
    shooting: (bool, u32),
    ammos: u32,
    starting_ammos: u32,
    score: u32,
    health: u8,
    max_health: u8,
    animation_cycle: u8,
    speed: f32,
    starting_speed: f32,
    movement_queue: VecDeque<Movement>,
    map_size: (f32, f32),
    shots: VecDeque<Bullet>,
    invicibility_frames: u32
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

        let bullet_sprite = graphics::Image::new(ctx, "/bullet.png")?;

        let fabien = Fabien {
            sprites: sprites,
            bullet_sprite: bullet_sprite,
            facing: "front".to_string(),
            hitbox: hitbox,
            shooting: (false, 0),
            ammos: 33,
            starting_ammos: 33,
            score: 0,
            health: 10,
            max_health: 10,
            animation_cycle: 0,
            speed: 0.8,
            starting_speed: 0.8,
            movement_queue: VecDeque::new(),
            map_size: (width, height),
            shots: VecDeque::<Bullet>::new(),
            invicibility_frames: 0
        };

        Ok(fabien)
    }

    pub fn draw(&mut self, ctx: &mut Context, frames: &u32, screen_size: &(f32, f32)) -> GameResult {
        //Update the invicibility_frames if Fabien is currently invicible
        if self.invicibility_frames > 0 { self.invicibility_frames -= 1; }

        // Update the camera view
        let camera_size: (f32, f32) = (screen_size.0 / 4.0, screen_size.1 / 4.0);
        let mut camera_x = self.hitbox.x - camera_size.0 / 2.0;
        let mut camera_y = self.hitbox.y - camera_size.1 / 2.0;

        if camera_x <= 0.0 { camera_x = 0.0; }
        else if camera_x >= self.map_size.0 - camera_size.0 {
            camera_x = self.map_size.0 - camera_size.0;
        }

        if camera_y <= 0.0 { camera_y = 0.0 }
        else if camera_y >= self.map_size.1 - camera_size.1 {
            camera_y = self.map_size.1 -  camera_size.1;
        }

        graphics::set_screen_coordinates(ctx, Rect::new(
            camera_x, camera_y,
            camera_size.0, camera_size.1
        ))?;

        // Update the character sprite, depending in what direction he's moving
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
        else { self.animation_cycle = 0; }

        if self.shots.len() > 0 {
            for b in self.shots.iter_mut() {
                b.draw(ctx)?;
            }
        }

        let sprite = self.sprites.get(&format!("{}_{}", self.facing, self.animation_cycle)).unwrap();
        graphics::draw(ctx, sprite, (Point2::new(self.hitbox.x, self.hitbox.y),))?;

        // Drawing remaining ammos
        const BULLET_SCALE: f32 = 0.2;
        let bullet_width = self.bullet_sprite.width() as f32;
        let bullet_height = self.bullet_sprite.height() as f32;
        let mut param = graphics::DrawParam::default()
            .scale(Vector2::new(0.5, 0.5));

        camera_x += 1.0;
        camera_y += 1.0;

        let mut i = 0;
        let mut j = 0;
        for _ in 0..self.ammos {
            param = param.dest(Point2::new(
                    camera_x + ((bullet_width + 2.0) * BULLET_SCALE) * i as f32,
                    camera_y + ((bullet_height + 2.0) * BULLET_SCALE) * j as f32
            ));
            graphics::draw(ctx, &self.bullet_sprite, param)?;
            if i % 10 == 0 && i != 0 {
                i = 0;
                j += 1;
            } else { i += 1; }
        }

        Ok(())
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
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

        if self.shooting.0 { self.shooting.1 += 1; }
        if self.shots.len() > 0 {
            let mut to_remove = vec![];
            for (i, b) in self.shots.iter_mut().enumerate() {
                if !b.update() { to_remove.push(i); }
            }
            for x in to_remove.iter() {
                self.shots.remove(*x);
            }
        }
        if self.shooting.1 > 25 { self.shooting.0 = false; self.shooting.1 = 0; }

        Ok(())
    }

    pub fn take_hit(&mut self) {
        if self.invicibility_frames <= 0 {
            self.health -= 1;
            self.invicibility_frames = 100;
        } 
    }

    pub fn add_to_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    pub fn key_down_event(&mut self, keycode: KeyCode, ctx: &mut Context) {
        match keycode {
            KeyCode::Z => self.movement_queue.push_back(Movement::Up),
            KeyCode::Q => self.movement_queue.push_back(Movement::Left),
            KeyCode::S => self.movement_queue.push_back(Movement::Down),
            KeyCode::D => self.movement_queue.push_back(Movement::Right),
            KeyCode::Space => {
                if !self.shooting.0 && self.ammos > 0 { 
                    self.shooting.0 = true;
                    self.ammos -= 1;

                    const BULLET_SPEED: f32 = 3.0;
                    let (mov, pos) = match &self.facing[..] {
                        "front" => (Movement::Down, (self.hitbox.x + 1.0, self.hitbox.y + 8.0)),
                        "back" => (Movement::Up, (self.hitbox.x + 6.0, self.hitbox.y + 9.0)),
                        "left" => (Movement::Left, (self.hitbox.x, self.hitbox.y + 7.0)),
                        "right" => (Movement::Right, (self.hitbox.x + 11.0, self.hitbox.y + 7.0)),
                        _ => (Movement::Down, (0.0, 0.0))
                    };

                    let bullet = Bullet::new(
                        ctx,
                        BULLET_SPEED,
                        mov,
                        Rect::new(pos.0, pos.1, 1.0, 1.0),
                        100,
                        self.map_size
                    ).unwrap();

                    self.shots.push_back(bullet);
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

    pub fn reset(&mut self, screen_size: (f32, f32)) {
        self.hitbox.x = screen_size.0 / 2.0;
        self.hitbox.y = screen_size.1 / 2.0;
        self.ammos = self.starting_ammos;
        self.speed = self.starting_speed;
        self.shooting = (false, 0);
        self.health = self.max_health;
        self.movement_queue.clear();
        self.facing = "front".to_string();
        self.score = 0;
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn get_shots(&self) -> &VecDeque<Bullet> {
        &self.shots
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}
