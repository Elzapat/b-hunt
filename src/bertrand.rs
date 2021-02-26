use ggez::{
    Context, GameResult, graphics,
    graphics::{
        Rect, DrawParam
    },
    nalgebra::{ Vector2, Point2 }
};
use std::collections::HashMap;
use rand::Rng;
use crate::map::Tree;
use crate::particle::Particle;

pub struct Bertrand {
    sprites: HashMap<String, graphics::Image>,
    facing: String,
    animation_cycle: u8,
    animation_time: f32,
    swinging: (bool, f32),
    hitbox: Rect,
    objective: Point2<f32>,
    is_in_tree: bool,
    particles: Vec<Particle>,
    speed: f32
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
            animation_time: 0.0,
            swinging: (false, 0.0),
            objective: Point2::new(0.0, 0.0),
            is_in_tree: false,
            particles: vec![],
            speed: 75.0
        };

        Ok(bertrand)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        if !self.swinging.0 {
            self.animation_time += dt;
            if self.animation_time > 1.0 / 6.0 {
                self.animation_time = 0.0;
                self.animation_cycle = (self.animation_cycle + 1) % 4;
            }
        } else {
            if self.swinging.1 < 0.25 { self.animation_cycle = 4; }
            else { self.animation_cycle = 5; }

            self.swinging.1 += dt;
        }

        let sprite = self.sprites.get(&format!("{}_{}", self.facing, self.animation_cycle)).unwrap();
        let param = DrawParam::default()
            .dest(Point2::new(self.hitbox.x - 3.0, self.hitbox.y));
        graphics::draw(ctx, sprite, param)?;

        for p in self.particles.iter() { p.draw(ctx)?; }

        Ok(())
    }

    pub fn update(&mut self, ctx: &mut Context, fabien_hitbox: Rect, trees: &Vec<Tree>) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        if self.swinging.0 { 
            for p in self.particles.iter_mut() { p.update(ctx); }
            self.particles.retain(|p| !p.is_dead());
            return Ok(());
        }

        let was_in_tree = self.is_in_tree;
        self.is_in_tree = false;
        for tree in trees.iter() {
            if self.hitbox.overlaps(&tree.get_hitbox()) && !was_in_tree {
                self.is_in_tree = true;
                self.spawn_leaf_particles(ctx)?;
                self.speed /= 8.0;
                break;
            }
        }

        if was_in_tree && !self.is_in_tree {
            self.spawn_leaf_particles(ctx)?; 
            self.speed *= 8.0;
        }

        let fabien_pos = (fabien_hitbox.x, fabien_hitbox.y);
        let next_move = self.speed * dt;

        if (self.hitbox.x < fabien_pos.0 + 30.0 &&
            self.hitbox.x > fabien_pos.0 - 30.0) &&
           (self.hitbox.y < fabien_pos.1 + 30.0 &&
            self.hitbox.y > fabien_pos.1 - 30.0)
        {
            self.move_towards(ctx, fabien_pos);

            if self.hitbox.x < fabien_pos.0 + next_move && self.hitbox.x > fabien_pos.0 - next_move {
                self.hitbox.x = fabien_pos.0;
            }
            if self.hitbox.y < fabien_pos.1 + next_move && self.hitbox.y > fabien_pos.1 - next_move {
                self.hitbox.y = fabien_pos.1;
            }
        } else {
            if self.animation_cycle == 0 {
                self.objective = Point2::new(fabien_pos.0, fabien_pos.1);
            }

            self.move_towards(ctx, (self.objective.x, self.objective.y));

            if self.hitbox.x < self.objective.x + next_move && self.hitbox.x > self.objective.x - next_move {
                self.hitbox.x = self.objective.x;
            }
            if self.hitbox.y < self.objective.y + next_move && self.hitbox.y > self.objective.y - next_move {
                self.hitbox.y = self.objective.y;
            }
        }

        for p in self.particles.iter_mut() { p.update(ctx); }
        self.particles.retain(|p| !p.is_dead());

        Ok(())
    }

    fn move_towards(&mut self, ctx: &mut Context, target: (f32, f32)) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        let mut dir = Vector2::new(0.0, 0.0);
        if target.0 < self.hitbox.x {
            dir.x = -1.0;
            self.facing = "left".to_string();
        } else if target.0 > self.hitbox.x {
            dir.x = 1.0;
            self.facing = "right".to_string();
        }

        if target.1 < self.hitbox.y {
            dir.y = -1.0;
            self.facing = "back".to_string();
        } else if target.1 > self.hitbox.y {
            dir.y = 1.0;
            self.facing = "front".to_string();
        }

        let vel_x = dir.x * self.speed * dt;
        let vel_y = dir.y * self.speed * dt;

        self.hitbox.x += vel_x;
        self.hitbox.y += vel_y;
    }

    fn spawn_leaf_particles(&mut self, ctx: &mut Context) -> GameResult {
        const NB_PARTICLES: usize = 15;

        let pos = (self.hitbox.x + self.hitbox.w / 2.0, self.hitbox.y + self.hitbox.h / 2.0);
        for _ in 0..NB_PARTICLES {
            let mut rng = rand::thread_rng();
            let r = 92 + (rng.gen_range(0..=30) - 15);
            let g = 169 + (rng.gen_range(0..=40) - 20);
            let b = 4 +  rng.gen_range(0..=20);
            let color = graphics::Color::from_rgb(r, g, b);
            let angle = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
            let size = rng.gen::<f32>() + 0.5; 
            let life = rng.gen::<f32>() * 1.0 + 1.0;
            const SPEED: f32 = 15.0;
            let coinflip = rand::random::<bool>();
            let rot_dir = if coinflip { -1.0 } else { 1.0 };
            let rot_speed = rot_dir * 20.0;

            self.particles.push(
                Particle::new(Point2::new(pos.0, pos.1), SPEED, rot_speed, angle, life, color, size, ctx)?
            );
        }

        Ok(())
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn is_dead(&self) -> bool {
        self.swinging.1 > 0.5
    }

    pub fn is_swinging(&self) -> bool {
        self.swinging.0
    }

    pub fn swing(&mut self) {
        self.swinging.0 = true;
    }
}
