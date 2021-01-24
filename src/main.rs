mod map; use map::Map;
mod fabien; use fabien::Fabien;
mod bertrand; use bertrand::Bertrand;
mod menu; use menu::Menu;
mod game_over; use game_over::GameOver;
pub mod utils; use utils::*;
pub mod bullet;

use ggez::{
    event, graphics, Context, GameResult,
    graphics::{ Rect, Drawable },
    event::*,
};
use std::io::prelude::*;
use std::fs::OpenOptions;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct MainState {
    screen_size: (f32, f32), 
    game_state: GameState,
    menu: Menu,
    game_over: GameOver,
    map: Map,
    fabien: Fabien,
    bertrands: Vec<Bertrand>,
    sec_since_last_bertrand: f64,
    time_passed: f64,
    frames: u32,
    wave: u32
}

impl MainState {
    fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<MainState> {
        let map = Map::new(ctx, width, height)?;
        let fabien = Fabien::new(ctx, width, height)?;
        let menu = Menu::new(ctx)?;
        let game_over = GameOver::new(ctx, 0, 0)?;

        let s = MainState {
            screen_size: (width, height),
            game_state: GameState::Menu,
            menu: menu,
            game_over: game_over,
            map: map,
            fabien: fabien,
            bertrands: Vec::<Bertrand>::new(),
            sec_since_last_bertrand: 0.0,
            time_passed: 0.0,
            frames: 0,
            wave: 1
        };
        Ok(s)
    }

    fn check_collisions(&mut self) {
        let fabien_hitbox = self.fabien.get_hitbox();

        // Check if any Bertrand is colliding with any of Fabien's bullets
        let mut nb_removed = 0;
        {
            let shots = self.fabien.get_shots(); 
            for bullet in shots.iter() {
                self.bertrands.retain(|bertrand| { 
                    if bertrand.get_hitbox().overlaps(&bullet.get_hitbox()) {
                        nb_removed += 1;
                        false 
                    } else { true }
                });
            }
        }
        self.fabien.add_to_score(nb_removed);

        // Check if any Bertrand is colliding with Fabien
        // I think I cannot use the retain methode because I want to execute
        // a function on Fabien if they're colliding, and retain won't let me
        // do that.
        // self.bertrands.retain(|bertrand| bertrand.get_hitbox().overlaps(&fabien_hitbox));
        let mut to_remove = vec![];
        for (i, bertrand) in self.bertrands.iter_mut().enumerate() {
            if bertrand.get_hitbox().overlaps(&fabien_hitbox) {
                self.fabien.take_hit();
                to_remove.push(i);
            }
        }
        for x in to_remove.iter() { self.bertrands.remove(*x); }
    }

    fn write_score(&self) -> u32 {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(false)
            .open("high_score.txt")
            .unwrap();

        let mut high_score = String::new();
        file.read_to_string(&mut high_score).unwrap();

        let high_score = match high_score.trim().to_string().parse::<u32>() {
            Ok(cur_high_score) => cur_high_score,
            Err(_) => 0
        };
        
        if self.fabien.get_score() > high_score {
            file.set_len(0).unwrap();
            file.seek(std::io::SeekFrom::Start(0)).unwrap();
            file.write_all(&self.fabien.get_score().to_string().into_bytes()).unwrap();

            self.fabien.get_score()
        } else { high_score }
    }

    fn bertrand_spawning(&mut self, ctx: &mut Context, fps: f64) -> GameResult {
        // If a minute passed since the last wave change, the wave changes
        if self.time_passed > (60 * self.wave) as f64 {
            println!("wave passed!");
            self.fabien.add_to_score(10 * self.wave);
            self.wave += 1;
        }

        let bertrand_spawning_rate: f32 = 800.0 / (0.8 * self.wave as f32);
        self.sec_since_last_bertrand += 1.0 / fps;

        let rand_nb = rand(bertrand_spawning_rate) as f64;

        if rand_nb - self.sec_since_last_bertrand < 0.0 {
            self.sec_since_last_bertrand = 0.0;
            let mut new_bertrand_pos: (f32, f32);
            println!("new bertrand spawned");

            loop {
                new_bertrand_pos = (rand(self.map.get_width()), rand(self.map.get_height()));
                if (new_bertrand_pos.0 < self.fabien.get_hitbox().x - 50.0 ||
                   new_bertrand_pos.0 > self.fabien.get_hitbox().x + 50.0) &&
                   (new_bertrand_pos.1 < self.fabien.get_hitbox().y - 50.0 ||
                   new_bertrand_pos.1 > self.fabien.get_hitbox().y + 50.0) { break; }
            }
            self.bertrands.push(Bertrand::new(ctx, Rect::new(
                new_bertrand_pos.0, new_bertrand_pos.1, 10.0, 10.0
            ))?);
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
                self.check_collisions();
                self.fabien.update(ctx)?;
                for b in self.bertrands.iter_mut() {
                    b.update((self.fabien.get_hitbox().x, self.fabien.get_hitbox().y))?;
                }

                let fps = ggez::timer::fps(ctx);
                self.time_passed += 1.0 / fps;

                self.bertrand_spawning(ctx, fps)?;

                // Check if Fabien is dead, if so it's Game Over
                if self.fabien.get_health() <= 0 {
                    let high_score = self.write_score();

                    self.game_over = GameOver::new(ctx, high_score, self.fabien.get_score())?;

                    graphics::set_screen_coordinates(ctx, 
                        Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1))?;
                    self.game_state = GameState::GameOver;
                }
            },
            GameState::GameOver => {
                self.game_over.update();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //println!("FPS: {}", ggez::timer::fps(ctx));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        match self.game_state {
            GameState::Menu => {
                self.map.draw(ctx)?;
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
                self.map.draw(ctx)?;
                self.fabien.draw(ctx, &self.frames, &self.screen_size)?;
                for b in self.bertrands.iter_mut() {
                    b.draw(ctx)?;
                }

                self.frames = (self.frames + 1) % 40;
            },
            GameState::GameOver => {
                self.map.draw(ctx)?;
                let shade_rect = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1),
                    graphics::Color::new(0.0, 0.0, 0.0, 0.9)
                ).unwrap();
                graphics::draw(ctx, &shade_rect, graphics::DrawParam::default())?;
                self.game_over.draw(ctx, self.screen_size)?;
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
                self.fabien.key_down_event(keycode, ctx);
            },
            GameState::GameOver => {
                self.time_passed = 0.0;
                self.wave = 1;
                self.fabien.reset(self.screen_size);
                self.game_state = GameState::Playing;
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
            GameState::GameOver => {}
        }
    }
}

pub fn main() -> GameResult {

    let mut cb = ggez::ContextBuilder::new("B-Hunt", "Elzapat");
    cb = cb.window_setup(ggez::conf::WindowSetup {
        title: "B-Hunt".to_string(),
        vsync: true,
        icon: "/Fabien_front_0.png".to_string(),
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
        path.push("Fabien");
        cb = cb.add_resource_path(&path);
    }

    let (mut ctx, mut event_loop) = cb.build()?;
    //graphics::set_resizable(&mut ctx, false)?;
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);
    //graphics::set_screen_coordinates(&mut ctx, graphics::Rect::new(
    //        0.0, 0.0, 100.0, 100.0))?;
    let state = &mut MainState::new(&mut ctx, width, height)?;
    event::run(&mut ctx, &mut event_loop, state)
}
