mod map; use map::Map;
mod fabien; use fabien::Fabien;
mod bertrand; use bertrand::Bertrand;
mod menu; use menu::Menu;
mod game_over; use game_over::GameOver;
pub mod powerup; use powerup::Powerup;
pub mod utils; use utils::*;
pub mod bullet;
pub mod particle;

extern crate mysql;
extern crate dotenv; use dotenv::dotenv;
extern crate rand;
extern crate serde_json;
extern crate serde;
extern crate reqwest;
extern crate sha1;
extern crate urlencoding;

use ggez::{
    event, graphics, Context, GameResult,
    graphics::Rect,
    event::*,
};

enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct MainState {
    screen_size: (f32, f32), 
    stats: Stats,
    game_state: GameState,
    menu: Menu,
    game_over: Option<GameOver>,
    map: Map,
    fabien: Fabien,
    bertrands: Vec<Bertrand>,
    powerups: Vec<Powerup>,
    sec_since_last_bertrand: f64,
    sec_since_last_powerup: f64,
    time_passed: f64,
    updated_this_frame: bool,
    wave: u32
}

impl MainState {
    fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<MainState> {
        let mut map = Map::new(ctx, width, height)?;
        let fabien = Fabien::new(ctx, width, height, map.get_trees())?;
        let menu = Menu::new(ctx)?;
        let game_over = None;


        let s = MainState {
            screen_size: (width, height),
            stats: Stats { bertrand_killed: 0, shots: 0, powerups_activated: 0, hits_taken: 0, time_played: 0 },
            game_state: GameState::Menu,
            menu: menu,
            game_over: game_over,
            map: map,
            fabien: fabien,
            bertrands: Vec::<Bertrand>::new(),
            powerups: Vec::<Powerup>::new(),
            sec_since_last_bertrand: 0.0,
            sec_since_last_powerup: 0.0,
            time_passed: 0.0,
            updated_this_frame: true,
            wave: 1
        };
        Ok(s)
    }

    fn check_collisions(&mut self) {
        let fabien_hitbox = self.fabien.get_hitbox();

        // Check if any Bertrand is colliding with any of Fabien's bullets
        // animation_frames: 0,
        // and let the bullet go through if Fabien has the powerup for that
        let mut nb_removed = 0;
        {
            use std::collections::VecDeque;
            use crate::bullet::Bullet;

            let shots: &mut VecDeque<Bullet> = self.fabien.get_shots(); 
            for bullet in shots.iter_mut() {
                self.bertrands.retain(|bertrand| { 
                    if bertrand.get_hitbox().overlaps(&bullet.get_hitbox()) {
                        nb_removed += 1;
                        bullet.hit_something();
                        false 
                    } else { true }
                });

                if bullet.get_nb_pierce() < 0 { 
                    bullet.set_life(0);
                    self.stats.shots += 1;
                }
            }
        }
        self.fabien.add_to_score(nb_removed);
        self.stats.bertrand_killed += nb_removed as u64;

        // Check if any Bertrand is colliding with Fabien
        // I think I cannot use the retain methode because I want to execute
        // a function on Fabien if they're colliding, and retain won't let me
        // do that.
        // self.bertrands.retain(|bertrand| bertrand.get_hitbox().overlaps(&fabien_hitbox));
        for bertrand in self.bertrands.iter_mut() {
            if bertrand.is_swinging() { continue; }
            if bertrand.get_hitbox().overlaps(&fabien_hitbox) {
                if self.fabien.take_hit() {
                    self.stats.hits_taken += 1;
                }
                bertrand.swing();
                break;
            }
        }
        self.bertrands.retain(|b| !b.is_dead());

        // Check if Fabien is colliding with a powerup
        let mut to_remove: Option<usize> = None;
        for (i, powerup) in self.powerups.iter().enumerate() {
            if powerup.get_hitbox().overlaps(&fabien_hitbox) {
                self.fabien.activate_powerup(powerup.get_powerup());
                self.stats.powerups_activated += 1;
                to_remove = Some(i);
                break;
            }
        }
        if let Some(x) = to_remove { self.powerups.remove(x); }

        // Check if a bullet is colliding with a tree
        let trees = self.map.get_trees();
        'main: for (i, bullet) in self.fabien.get_shots().iter().enumerate() {
            for tree in trees.iter() {
                if bullet.get_hitbox().overlaps(&tree.get_hitbox()) {
                    to_remove = Some(i); 
                    break 'main;
                }
            }
        }
        if let Some(x) = to_remove { self.fabien.get_shots().remove(x); }
    }

    fn bertrand_spawning(&mut self, ctx: &mut Context, fps: f64) -> GameResult {
        // If a minute passed since the last wave change, the wave changes
        if self.time_passed > (60 * self.wave) as f64 {
            println!("wave passed!");
            self.fabien.add_to_score(10 * self.wave);
            self.wave += 1;
        }

        let bertrand_spawning_rate: f32 = 450.0 / (0.8 * self.wave as f32);
        self.sec_since_last_bertrand += 1.0 / fps;

        let rand_nb = rand(bertrand_spawning_rate) as f64;

        if rand_nb - self.sec_since_last_bertrand < 0.0 {
            self.sec_since_last_bertrand = 0.0;
            let mut new_bertrand_pos: (f32, f32);

            loop {
                new_bertrand_pos = (rand(self.map.get_width()), rand(self.map.get_height()));
                let mut not_in_tree = true;
                for tree in self.map.get_trees().iter() {
                    if tree.get_hitbox().contains(
                        ggez::mint::Point2 {
                            x: new_bertrand_pos.0, y: new_bertrand_pos.1 })
                    {
                        not_in_tree = false;
                        break;
                    }
                }
                if (new_bertrand_pos.0 < self.fabien.get_hitbox().x - 200.0 ||
                   new_bertrand_pos.0 > self.fabien.get_hitbox().x + 200.0) &&
                   (new_bertrand_pos.1 < self.fabien.get_hitbox().y - 200.0 ||
                   new_bertrand_pos.1 > self.fabien.get_hitbox().y + 200.0) &&
                   not_in_tree { break; }
            }
            self.bertrands.push(Bertrand::new(ctx, Rect::new(
                new_bertrand_pos.0, new_bertrand_pos.1, 8.0, 16.0
            ))?);
        }

        Ok(())
    }

    fn powerup_spawning(&mut self, ctx: &mut Context, fps: f64) -> GameResult {
        self.sec_since_last_powerup += 1.0 / fps;
        let powerup_spawn_rate: f32 = 6000.0 / (0.4 * self.wave as f32);

        let rand_nb = rand(powerup_spawn_rate) as f64;

        if rand_nb - self.sec_since_last_powerup < 0.0 {
            self.powerups.push(Powerup::new(ctx, self.screen_size)?);
            self.sec_since_last_powerup = 0.0;
        }

        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        match self.game_state {
            GameState::Menu => {
                self.menu.update();
            },
            GameState::Playing => {
                const DESIRED_FPS: u32 = 60;
                while ggez::timer::check_update_time(ctx, DESIRED_FPS) {
                    self.updated_this_frame = true;
                    let fps = ggez::timer::fps(ctx);

                    self.check_collisions();
                    self.fabien.update(ctx, fps, self.map.get_trees())?;
                    for b in self.bertrands.iter_mut() {
                        b.update(ctx, self.fabien.get_hitbox(), self.map.get_trees())?;
                    }
                    for p in self.powerups.iter_mut() {
                        p.update(ctx, self.time_passed, fps)?;
                    }

                    self.time_passed += 1.0 / fps;

                    self.bertrand_spawning(ctx, fps)?;
                    self.powerup_spawning(ctx, fps)?;

                    // Check if Fabien is dead, if so it's Game Over
                    if self.fabien.get_health() <= 0 {
                        self.stats.time_played += self.time_passed as u64;
                        self.game_over = Some(GameOver::new(ctx,
                            self.fabien.get_score(), self.stats, self.screen_size)?);

                        graphics::set_screen_coordinates(ctx, 
                            Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1))?;
                        self.game_state = GameState::GameOver;
                    }
                }
            },
            GameState::GameOver => {
                self.game_over.as_ref().unwrap().update();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("FPS: {}", ggez::timer::fps(ctx));
        match self.game_state {
            GameState::Menu => {
                graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                self.map.draw(ctx)?;
                self.map.draw_trees_before(ctx)?;
                self.map.draw_trees_after(ctx)?;
                let shade_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1),
                    graphics::Color::new(0.0, 0.0, 0.0, 0.9)
                ).unwrap();
                graphics::draw(ctx, &shade_rect, graphics::DrawParam::default())?;
                self.menu.draw(ctx, self.screen_size)?;
            },
            GameState::Playing => {
                    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                    self.map.draw(ctx)?;
                    for p in self.powerups.iter() {
                        p.draw(ctx)?;
                    }
                    for b in self.bertrands.iter_mut() {
                        b.draw(ctx)?;
                    }
                    self.map.draw_trees_before(ctx)?;
                    self.fabien.draw(ctx)?;
                    self.map.draw_trees_after(ctx)?;
                    self.fabien.draw_infos(ctx, self.screen_size)?;
            },
            GameState::GameOver => {
                graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                self.map.draw(ctx)?;
                self.map.draw_trees_before(ctx)?;
                self.map.draw_trees_after(ctx)?;
                let shade_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1),
                    graphics::Color::new(0.0, 0.0, 0.0, 0.9)
                ).unwrap();
                graphics::draw(ctx, &shade_rect, graphics::DrawParam::default())?;
                self.game_over.as_ref().unwrap().draw(ctx, self.screen_size)?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match self.game_state {
            GameState::Menu => {
                self.game_state = GameState::Playing;
            },
            GameState::Playing => {
                if let event::KeyCode::Space = keycode {
                    if !self.fabien.is_shooting() && self.fabien.get_nb_ammos() > 0 {
                        self.stats.shots += 1;
                    }
                }
                self.fabien.key_down_event(keycode, ctx).unwrap();
            },
            GameState::GameOver => {

            }
        }
    } 

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) { 
        match self.game_state {
            GameState::Menu => {

            },
            GameState::Playing => {
                self.fabien.key_up_event(keycode); 
            },
            GameState::GameOver => {

            }
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        match self.game_state {
            GameState::Menu => {

            },
            GameState::Playing => {

            },
            GameState::GameOver => { 
                self.time_passed = 0.0;
                self.wave = 1;
                self.stats = Stats { bertrand_killed: 0, shots: 0,
                    powerups_activated: 0, hits_taken: 0, time_played: 0 };
                self.fabien.reset(self.screen_size);
                self.bertrands.clear();
                self.game_state = GameState::Playing;
            }
        }

    }
}

fn main() -> GameResult {
    dotenv().ok();
    let mut cb = ggez::ContextBuilder::new("B-Hunt", "Elzapat");
    cb = cb.window_setup(ggez::conf::WindowSetup {
        title: "B-Hunt".to_string(),
        vsync: true,
        icon: "/Fabien/Fabien_front_0.png".to_string(),
        ..Default::default()
    });
    let (width, height) = (1000.0, 750.0);
    cb = cb.window_mode(ggez::conf::WindowMode {
        width: width,
        height: height,
        resizable: false,
        ..Default::default()
    });
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(&path);
    }

    let (mut ctx, mut event_loop) = cb.build()?;
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let state = &mut MainState::new(&mut ctx, width, height)?;
    event::run(&mut ctx, &mut event_loop, state)
}
