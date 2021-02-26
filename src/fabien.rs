use std::collections::{ HashMap, VecDeque };
use rand::Rng;
use ggez::{
    graphics, Context, GameResult,
    nalgebra::{ Point2, Vector2 },
    graphics::Rect,
    event::KeyCode,
    input::mouse::MouseButton
};
use crate::utils::Movement;
use crate::bullet::Bullet;
use crate::powerup::Powerups;
use crate::text::Text;
use crate::particle::Particle;
use crate::map::Tree;

// Fabien is the player
pub struct Fabien {
    sprites: HashMap<String, graphics::Image>,
    bullet_sprite: graphics::Image,
    sandwich_sprite: graphics::Image,
    moldy_sandwich_sprite: graphics::Image,
    piercing_bullet_sprite: graphics::Image,
    facing: String,
    hitbox: Rect,
    camera: Rect,
    shooting: (bool, f32),
    ammos: u32,
    starting_ammos: u32,
    score: u32,
    health: u8,
    max_health: u8,
    animation_cycle: u8,
    animation_time: f32,
    speed: f32,
    starting_speed: f32,
    active_powerup: Option<Powerups>,
    powerup_sprite: Option<graphics::Image>,
    movement_queue: VecDeque<Movement>,
    map_size: (f32, f32),
    shots: VecDeque<Bullet>,
    particles: Vec<Particle>,
    invicibility_frames: u32
}

impl Fabien {
    pub fn new(ctx: &mut Context, trees: &Vec<Tree>, map_size: (f32, f32), screen_size: (f32, f32)) -> GameResult<Fabien> {
        let mut hitbox = Rect::new(map_size.0 / 2.0, map_size.1 / 2.0, 8.0, 16.0);
        let mut overlaps;
        'main: loop {
            overlaps = false;
            for tree in trees.iter() {
                if hitbox.overlaps(&tree.get_hitbox()) {
                    hitbox.x -= 10.0;
                    hitbox.y -= 10.0;
                    overlaps = true;
                    break;
                }
            } 
            if !overlaps { break 'main; }
        }
        let mut sprites = HashMap::new();

        for facing in ["front", "back", "right", "left"].iter() {
            for i in 0..=4 {
                let image = graphics::Image::new(ctx, format!("/Fabien/Fabien_{}_{}.png", facing, i))?;
                sprites.insert(format!("{}_{}", facing, i), image);
            }
        }

        let camera_size = (screen_size.0 / 4.5, screen_size.1 / 4.5);
        let camera = Rect::new(
            hitbox.x - camera_size.0 / 2.0,
            hitbox.y - camera_size.1 / 2.0,
            camera_size.0, camera_size.1
        );

        let bullet_sprite = graphics::Image::new(ctx, "/bullet.png")?;
        let sandwich_sprite = graphics::Image::new(ctx, "/sandwich.png")?;
        let moldy_sandwich_sprite = graphics::Image::new(ctx, "/moldy_sandwich.png")?;
        let piercing_bullet_sprite = graphics::Image::new(ctx, "/piercing_bullet.png")?;

        let fabien = Fabien {
            sprites: sprites,
            bullet_sprite: bullet_sprite,
            sandwich_sprite: sandwich_sprite,
            moldy_sandwich_sprite: moldy_sandwich_sprite,
            piercing_bullet_sprite: piercing_bullet_sprite,
            facing: "front".to_string(),
            hitbox: hitbox,
            camera: camera,
            shooting: (false, 0.0),
            ammos: 44,
            starting_ammos: 44,
            score: 0,
            health: 10,
            max_health: 10,
            animation_cycle: 0,
            animation_time: 0.0,
            speed: 50.0,
            starting_speed: 50.0,
            active_powerup: None,
            powerup_sprite: None,
            movement_queue: VecDeque::new(),
            map_size: map_size,
            shots: VecDeque::<Bullet>::new(),
            particles: vec![],
            invicibility_frames: 0
        };

        Ok(fabien)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Update the character sprite, depending in what direction he's moving
        if self.movement_queue.len() > 0 && !self.shooting.0 {
            match self.movement_queue.back().unwrap() {
                Movement::Up => self.facing = "back".to_string(),
                Movement::Left => self.facing = "left".to_string(),
                Movement::Down => self.facing = "front".to_string(),
                Movement::Right => self.facing = "right".to_string()
            }
            self.animation_time += ggez::timer::delta(ctx).as_secs_f32();
            if self.animation_time > 1.0 / 6.0 {
                self.animation_time = 0.0;
                self.animation_cycle = (self.animation_cycle + 1) % 4;
            }
        } else if self.shooting.0 { self.animation_cycle = 4; }
        else { self.animation_cycle = 0; }

        for b in self.shots.iter_mut() { b.draw(ctx)?; }

        let sprite = self.sprites.get(&format!("{}_{}", self.facing, self.animation_cycle)).unwrap();
        let param = graphics::DrawParam::default()
            .dest(Point2::new(self.hitbox.x - 3.0, self.hitbox.y));
        graphics::draw(ctx, sprite, param)?;

        Ok(())
    }

    pub fn draw_infos(&mut self, ctx: &mut Context) -> GameResult {
        // Update the camera view
        self.camera.x = self.hitbox.x - self.camera.w / 2.0;
        self.camera.y = self.hitbox.y - self.camera.h / 2.0;

        if self.camera.x <= 0.0 { self.camera.x = 0.0; }
        else if self.camera.x >= self.map_size.0 - self.camera.w {
            self.camera.x = self.map_size.0 - self.camera.w;
        }

        if self.camera.y <= 0.0 { self.camera.y = 0.0 }
        else if self.camera.y >= self.map_size.1 - self.camera.h {
            self.camera.y = self.map_size.1 - self.camera.h;
        }

        graphics::set_screen_coordinates(ctx, self.camera)?;

        // Drawing remaining ammos
        {
            const BULLET_SCALE: f32 = 0.7;
            const BULLET_SPACING: f32 = BULLET_SCALE / 1.8;
            let bullet_width = self.bullet_sprite.width() as f32;
            let bullet_height = self.bullet_sprite.height() as f32;
            let bullet_origin = (self.camera.x + 1.0, self.camera.y + 1.0);
            let mut param = graphics::DrawParam::default()
                    .scale(Vector2::new(BULLET_SCALE, BULLET_SCALE));

            let bullet_sprite = if let Some(Powerups::PiercingBullet(_)) = self.active_powerup {
                &self.piercing_bullet_sprite
            } else {
                &self.bullet_sprite
            };

            let mut i = 0;
            let mut j = 0;
            for _ in 0..self.ammos {
                param = param.dest(Point2::new(
                        bullet_origin.0 + (bullet_width * BULLET_SPACING) * i as f32,
                        bullet_origin.1 + (bullet_height * BULLET_SPACING) * j as f32
                    ));
                graphics::draw(ctx, bullet_sprite, param)?;
                if i % 10 == 0 && i != 0 {
                    i = 0;
                    j += 1;
                } else { i += 1; }
            }
        }

        // Drawing health
        {
            const SANDWICH_SCALE: f32 = 0.065;
            const SANDWICH_SPACING: f32 = SANDWICH_SCALE / 1.5;
            let sandwich_width = self.sandwich_sprite.width() as f32;
            let sandwich_origin = (self.camera.x + self.camera.w - 1.0, self.camera.y - 1.0);
            let mut param = graphics::DrawParam::default()
                .scale(Vector2::new(SANDWICH_SCALE, SANDWICH_SCALE));

            for i in (0..self.max_health).rev() {
                param = param.dest(Point2::new(
                    sandwich_origin.0 - sandwich_width * SANDWICH_SCALE 
                        - (sandwich_width * SANDWICH_SPACING) * i as f32,
                    sandwich_origin.1
                ));
                let to_draw = if i + 1 > self.health {
                    &self.moldy_sandwich_sprite
                } else { &self.sandwich_sprite };

                graphics::draw(ctx, to_draw, param)?;
            }
        }

        // If Fabien has an active powerup, display the icon with the remaining time
        if let Some(powerup) = &self.active_powerup {
            let timer = match powerup {
                Powerups::SpeedBoost((time, _)) => time.ceil(),
                Powerups::PiercingBullet((time, _)) => time.ceil(),
                _ => 0.0
            };
            
            const SCALE: f32 = 0.1;
            let sprite = self.powerup_sprite.as_ref().unwrap();
            let sprite_pos = Point2::new(self.camera.x + self.camera.w - sprite.width() as f32 - 1.0,
                    self.camera.y + self.camera.h - sprite.height() as f32 - 1.0);

            let mut timer_text = Text::new(ctx, timer.to_string(), "/Fonts/arial_narrow_7.ttf".to_string(),
                                        100.0, graphics::Color::from_rgb(255, 255, 255))?;
            timer_text.set_pos(Point2::new(sprite_pos.x - timer_text.width(ctx) * SCALE - SCALE * 20.0,
                    sprite_pos.y + sprite.height() as f32 / 2.0 - (timer_text.height(ctx) * SCALE / 2.0)));

            graphics::queue_text(ctx, timer_text.get_ggez_text(), Point2::new(0.0, 0.0), None);
            graphics::draw_queued_text(ctx, graphics::DrawParam::new()
                .scale(ggez::nalgebra::Vector2::new(SCALE, SCALE))
                .dest(timer_text.get_pos()), None, graphics::FilterMode::Nearest)?;

            graphics::draw(ctx, sprite, graphics::DrawParam::default().dest(sprite_pos))?;
        }

        // Drawing particles
        for p in self.particles.iter() { p.draw(ctx)?; }

        // let hitbox = graphics::Mesh::new_rectangle(
        //     ctx,
        //     graphics::DrawMode::stroke(0.5),
        //     self.hitbox,
        //     graphics::Color::from_rgb(0, 0, 0)
        // )?;
        // graphics::draw(ctx, &hitbox, graphics::DrawParam::new().dest(Point2::new(0.0, 0.0)))?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, trees: &mut Vec<Tree>) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Update the invicibility_frames if Fabien is currently invicible
        if self.invicibility_frames > 0 { self.invicibility_frames -= 1; }

        let mut dir = Vector2::new(0.0, 0.0);
        if self.movement_queue.len() > 0 && !self.shooting.0 {
            match self.movement_queue.back() {
                Some(Movement::Up) => dir = Vector2::new(0.0, -1.0),
                Some(Movement::Left) => dir = Vector2::new(-1.0, 0.0),
                Some(Movement::Down) => dir = Vector2::new(0.0, 1.0),
                Some(Movement::Right) => dir = Vector2::new(1.0, 0.0),
                None => {}
            }
        }

        let vel_x = dir.x * self.speed * dt;
        let vel_y = dir.y * self.speed * dt;
        self.hitbox.x += vel_x;
        self.hitbox.y += vel_y;

        // Check if Fabien is out of the map, is so put him back at the edge
        if self.hitbox.x < 0.0 { self.hitbox.x = 0.0; }
        else if self.hitbox.x + self.hitbox.w > self.map_size.0 { self.hitbox.x = self.map_size.0 - self.hitbox.w; }
        if self.hitbox.y < 0.0 { self.hitbox.y = 0.0; }
        else if self.hitbox.y + self.hitbox.h > self.map_size.1 { self.hitbox.y = self.map_size.1 - self.hitbox.h; }

        // Check if Fabien is now in a tree, if so move him back
        for tree in trees.iter_mut() {
            let tree_hitbox = tree.get_hitbox();
            // If the tree is more than 50 units away from the player,
            // we don't consider it.
            if (tree_hitbox.x < self.hitbox.x - 50.0 ||
                tree_hitbox.x > self.hitbox.x + 50.0) &&
               (tree_hitbox.y < self.hitbox.y - 50.0 ||
                tree_hitbox.y > self.hitbox.y + 50.0)
                { continue; }

            if self.hitbox.y > tree_hitbox.y {
                tree.draw_before_fabien(true);
            } else if !self.hitbox.overlaps(&tree_hitbox) {
                tree.draw_before_fabien(false);
            } else {
                self.hitbox.x -= vel_x;
                self.hitbox.y -= vel_y;
            }
        }

        // Update the time left of the speed boost and piercing bullets powerups
        match self.active_powerup {
            Some(Powerups::SpeedBoost((ref mut time_left, _))) => {
                if time_left <= &mut 0.0 { self.active_powerup = None; }
                else { *time_left -= dt; }
            },
            Some(Powerups::PiercingBullet((ref mut time_left, _))) => {
                if time_left <= &mut 0.0 { self.active_powerup = None; }
                else { *time_left -= dt; }
            },
            _ => self.speed = self.starting_speed
        }

        if self.shooting.0 { self.shooting.1 += dt; }
        if self.shots.len() > 0 {
            let mut to_remove: Option<usize> = None;
            for (i, b) in self.shots.iter_mut().enumerate() {
                if !b.update(ctx) { 
                    to_remove = Some(i);
                    break;
                }
            }
            if let Some(x) = to_remove { self.shots.remove(x); }
        }
        if self.shooting.1 > 0.4 { self.shooting.0 = false; self.shooting.1 = 0.0; }

        // Update the particles
        for p in self.particles.iter_mut() { p.update(ctx); }
        self.particles.retain(|p| !p.is_dead());

        Ok(())
    }

    pub fn take_hit(&mut self) -> bool {
        if self.invicibility_frames <= 0 {
            self.health -= 1;
            self.invicibility_frames = 30;
            true
        } else { false }
    }

    pub fn add_to_score(&mut self, to_add: u32) {
        self.score += to_add;
    }

    pub fn activate_powerup(&mut self, ctx: &mut Context, powerup: Powerups) -> GameResult {
        match powerup {
            Powerups::Heal(health) => {
                if self.health + health > self.max_health {
                    self.health = self.max_health;
                } else {
                    self.health += health;
                }
            },
            Powerups::AmmoRestock(nb_ammos) => self.ammos += nb_ammos,
            Powerups::SpeedBoost((new_time, speed_mult)) => {
                self.active_powerup = Some(powerup);
                self.speed *= speed_mult;
                if self.speed > 170.0 {
                    self.speed = 170.0;
                    if let Some(Powerups::SpeedBoost((ref mut timer, _))) = self.active_powerup {
                        *timer += new_time;
                    }
                }
                self.powerup_sprite = Some(graphics::Image::new(ctx, "/speed_powerup.png")?);
            },
            Powerups::PiercingBullet((new_time, _)) => {
                if let Some(Powerups::SpeedBoost(_)) = self.active_powerup {
                    self.speed = self.starting_speed;
                }

                if let Some(Powerups::PiercingBullet((ref mut time, _))) = self.active_powerup {
                    *time += new_time;
                } else {
                    self.active_powerup = Some(powerup);
                    self.powerup_sprite = Some(graphics::Image::new(ctx, "/piercing_bullet.png")?);
                }
            }
        }

        Ok(())
    }

    pub fn key_down_event(&mut self, keycode: KeyCode) -> GameResult {
        match keycode {
            KeyCode::Z => self.movement_queue.push_back(Movement::Up),
            KeyCode::Q => self.movement_queue.push_back(Movement::Left),
            KeyCode::S => self.movement_queue.push_back(Movement::Down),
            KeyCode::D => self.movement_queue.push_back(Movement::Right),
            _ => {}
        }

        Ok(())
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

    pub fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton,
        x: f32, y: f32, screen_size: (f32, f32))
    {
        if let MouseButton::Left = button {
            let ratio_x = screen_size.0 / self.camera.w;
            let ratio_y = screen_size.1 / self.camera.h;
            self.shoot(ctx, self.camera.x + x / ratio_x, self.camera.y + y / ratio_y).unwrap();
        }
    }

    pub fn resize_event(&mut self, width: f32, height: f32) {
        self.camera.w = width / 4.5;
        self.camera.h = height / 4.5;
    }

    fn shoot(&mut self, ctx: &mut Context, x: f32, y: f32) -> GameResult {
        if !self.shooting.0 && self.ammos > 0 { 
            self.shooting.0 = true;
            self.ammos -= 1;

            const BULLET_SPEED: f32 = 300.0;
            let pos = match &self.facing[..] {
                "front" => (self.hitbox.x + 1.0, self.hitbox.y + 8.0),
                "back" => (self.hitbox.x + 6.0, self.hitbox.y + 9.0),
                "left" => (self.hitbox.x - 3.0, self.hitbox.y + 7.0),
                "right" => (self.hitbox.x + 11.0, self.hitbox.y + 7.0),
                _ => unreachable!()
            };

            let angle = (y - pos.1).atan2(x - pos.0);

            let nb_pierce = if let Some(Powerups::PiercingBullet((_, nb))) = self.active_powerup {
                nb
            } else { 0 };

            let bullet = Bullet::new(
                ctx,
                BULLET_SPEED,
                angle,
                Rect::new(pos.0, pos.1, 1.0, 1.0),
                nb_pierce as i8,
                5.0
            ).unwrap();

            self.shots.push_back(bullet);

            const NB_PARTICLES: usize = 10;

            // Spawning particles
            for _ in 0..NB_PARTICLES {
                let mut rng = rand::thread_rng();
                let r = 100;// + (rng.gen_range(0..=80) - 40);
                let g = 100;// + (rng.gen_range(0..=80) - 40);
                let b = 100;// +  (rng.gen_range(0..=20) - 10);
                let color = graphics::Color::from_rgb(r, g, b);
                let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
                let size = rng.gen::<f32>() + 0.5; 
                let life = rng.gen::<f32>() * 1.0 + 1.0;
                const SPEED: f32 = 10.0;
                let coinflip = rand::random::<bool>();
                let rot_dir = if coinflip { -1.0 } else { 1.0 };
                let rot_speed = rot_dir * 6.0;

                self.particles.push(
                    Particle::new(Point2::new(pos.0, pos.1), SPEED, rot_speed, angle, life, color, size, ctx)?
                );
            }
        }

        Ok(())
    }

    pub fn is_shooting(&self) -> bool {
        self.shooting.0
    }

    pub fn reset(&mut self, screen_size: (f32, f32)) {
        self.hitbox.x = screen_size.0 / 2.0;
        self.hitbox.y = screen_size.1 / 2.0;
        self.ammos = self.starting_ammos;
        self.speed = self.starting_speed;
        self.shooting = (false, 0.0);
        self.health = self.max_health;
        self.movement_queue.clear();
        self.facing = "front".to_string();
        self.score = 0;
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn get_shots(&mut self) -> &mut VecDeque<Bullet> {
        &mut self.shots
    }

    pub fn get_health(&self) -> u8 {
        self.health
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_nb_ammos(&self) -> u32 {
        self.ammos
    }

    pub fn clear_movement(&mut self) {
        self.movement_queue.clear();
    }

    pub fn get_camera(&self) -> Rect {
        self.camera
    }

    pub fn set_health(&mut self, health: u8) {
        self.health = health;
    }
}
