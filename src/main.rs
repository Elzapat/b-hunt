mod map; use map::Map;
mod fabien; use fabien::Fabien;
mod bertrand; use bertrand::Bertrand;
mod menu; use menu::Menu;
mod game_over; use game_over::GameOver;
mod pause; use pause::Pause;
pub mod powerup; use powerup::Powerup;
pub mod utils; use utils::*;
pub mod bullet;
pub mod particle;
pub mod button;
pub mod text; use text::Text;

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
    nalgebra::Point2,
    input::mouse::MouseButton
};

enum GameState {
    Menu,
    Playing,
    GameOver,
    Pause,
}

struct MainState {
    screen_size: (f32, f32), 
    map_size: (f32, f32),
    fullscreen: bool,
    stats: Stats,
    game_state: GameState,
    menu: Menu,
    game_over: Option<GameOver>,
    pause: Option<Pause>,
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
        let map_size = (2000.0, 2000.0);
        let mut map = Map::new(ctx, map_size.0, map_size.1)?;
        let fabien = Fabien::new(ctx, map.get_trees(), map_size, (width, height))?;
        let menu = Menu::new(ctx, (width, height))?;
        let game_over = None;

        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height))?;
        // graphics::set_mode(ctx, ggez::conf::WindowMode {
        //     width: width,
        //     height: height,
        //     maximized: false,
        //     resizable: false,
        //     ..Default::default()
        // }).unwrap();

        let s = MainState {
            screen_size: (width, height),
            map_size: map_size,
            fullscreen: true,
            stats: Stats { bertrand_killed: 0, shots: 0, powerups_activated: 0, hits_taken: 0, time_played: 0 },
            game_state: GameState::Menu,
            menu: menu,
            game_over: game_over,
            pause: None,
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

    fn check_collisions(&mut self, ctx: &mut Context) -> GameResult {
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
                    bullet.set_life(0.0);
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
                self.fabien.activate_powerup(ctx, powerup.get_powerup())?;
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

        Ok(())
    }

    fn bertrand_spawning(&mut self, ctx: &mut Context, fps: f64) -> GameResult {
        // If a minute passed since the last wave change, the wave changes
        if self.time_passed > (60 * self.wave) as f64 {
            self.fabien.add_to_score(10 * self.wave);
            self.wave += 1;
        }

        let bertrand_spawning_rate: f32 = 650.0 / (0.9 * self.wave as f32);
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
        let powerup_spawn_rate: f32 = 3500.0 / (0.6 * (self.wave + 1) as f32);

        let rand_nb = rand(powerup_spawn_rate) as f64;

        if rand_nb - self.sec_since_last_powerup < 0.0 {
            self.powerups.push(Powerup::new(ctx, self.map_size)?);
            self.sec_since_last_powerup = 0.0;
        }

        Ok(())
    }

    fn draw_infos(&self, ctx: &mut Context) -> GameResult {
        let minutes = (self.time_passed / 60.0).floor();
        let seconds = (self.time_passed - minutes * 60.0).floor();
        let infos = format!("{:02}:{:02}\nVague {}\nScore {}",
                    minutes, seconds, self.wave, self.fabien.get_score());

        let mut infos_text = Text::new(ctx, infos, "/Fonts/arial_narrow_7.ttf".to_string(),
                                    100.0, graphics::Color::from_rgb(255, 255, 255))?;
        infos_text.set_pos(Point2::new(self.fabien.get_camera().x + 1.0,
                self.fabien.get_camera().y + self.fabien.get_camera().h - infos_text.height(ctx) * 0.07));

        graphics::queue_text(ctx, infos_text.get_ggez_text(), Point2::new(0.0, 0.0), None);
        graphics::draw_queued_text(ctx, graphics::DrawParam::new()
            .scale(ggez::nalgebra::Vector2::new(0.07, 0.07))
            .dest(infos_text.get_pos()), None, graphics::FilterMode::Nearest)?;

        Ok(())
    }

    fn shade_rect(&self, ctx: &mut Context) -> GameResult {
        let (width, height) = if self.map_size.0 > self.screen_size.0 { 
            self.map_size
        } else {
            self.screen_size
        };

        let shade_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, width, height),
            graphics::Color::new(0.0, 0.0, 0.0, 0.9)
        ).unwrap();
        graphics::draw(ctx, &shade_rect, graphics::DrawParam::default())?;

        Ok(())
    }

    fn reset(&mut self) {
        self.time_passed = 0.0;
        self.wave = 1;
        self.stats = Stats { bertrand_killed: 0, shots: 0,
            powerups_activated: 0, hits_taken: 0, time_played: 0 };
        self.fabien.reset(self.screen_size);
        self.bertrands.clear();
        self.powerups.clear();
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        match self.game_state {
            GameState::Menu => {
                self.menu.update();
            },
            GameState::Playing => {
                self.updated_this_frame = true;
                let fps = ggez::timer::fps(ctx);

                self.check_collisions(ctx)?;
                self.fabien.update(ctx, self.map.get_trees())?;
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
            },
            GameState::GameOver => {
                self.game_over.as_ref().unwrap().update();
            },
            GameState::Pause => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // println!("FPS: {}", ggez::timer::fps(ctx));
        graphics::clear(ctx, graphics::Color::from_rgb(104, 159, 56));
        match self.game_state {
            GameState::Menu => {
                self.map.draw(ctx)?;
                self.map.draw_trees_before(ctx)?;
                self.map.draw_trees_after(ctx)?;
                self.shade_rect(ctx)?;
                self.menu.draw(ctx)?;
            },
            GameState::Playing => {
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
                self.fabien.draw_infos(ctx)?;
                self.draw_infos(ctx)?;
            },
            GameState::GameOver => {
                self.map.draw(ctx)?;
                self.map.draw_trees_before(ctx)?;
                self.map.draw_trees_after(ctx)?;
                self.shade_rect(ctx)?;
                self.game_over.as_ref().unwrap().draw(ctx)?;
            },
            GameState::Pause => {
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
                self.fabien.draw_infos(ctx)?;
                self.draw_infos(ctx)?;
                self.shade_rect(ctx)?;
                self.pause.as_ref().unwrap().draw(ctx, self.fabien.get_camera())?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        match self.game_state {
            GameState::Menu => {},
            GameState::Playing => {
                if let event::KeyCode::Space = keycode {
                    if !self.fabien.is_shooting() && self.fabien.get_nb_ammos() > 0 {
                        self.stats.shots += 1;
                    }
                }
                self.fabien.key_down_event(keycode).unwrap();

                if let event::KeyCode::Escape = keycode {
                    self.fabien.clear_movement();
                    graphics::set_screen_coordinates(ctx,
                        graphics::Rect::new(0.0, 0.0, self.screen_size.0, self.screen_size.1)).unwrap();
                    self.pause = Some(Pause::new(ctx, self.fabien.get_camera()).unwrap());
                    self.game_state = GameState::Pause;
                }
            },
            GameState::GameOver => {},
            GameState::Pause => {
                if let event::KeyCode::Escape = keycode {
                    self.game_state = GameState::Playing;
                }
            }
        }

        match keycode {
            event::KeyCode::F11 => {
                if self.fullscreen {
                    self.fullscreen = false;
                    graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::Windowed).unwrap();
                    ggez::graphics::set_mode(ctx, ggez::conf::WindowMode {
                        resizable: true,
                        maximized: true,
                        min_width: 800.0,
                        min_height: 600.0,
                        ..Default::default()
                    }).unwrap();
                } else {
                    self.fullscreen = true;
                    graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::True).unwrap();
                }
            }
            _ => {}
        }
    } 

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) { 
        match self.game_state {
            GameState::Menu => {},
            GameState::Playing => {
                self.fabien.key_up_event(keycode); 
            },
            GameState::GameOver => {},
            GameState::Pause => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match self.game_state {
            GameState::Menu => {
                if self.menu.mouse_button_down_event(ctx, x, y, button, self.screen_size) == 1 {
                    self.game_state = GameState::Playing;
                }
            },
            GameState::Playing => {
                self.fabien.mouse_button_down_event(ctx, button, x, y, self.screen_size);
            },
            GameState::GameOver => { 
                match self.game_over.as_ref().unwrap().mouse_button_down_event(button, x, y) {
                    1 => {
                        self.reset();
                        self.game_state = GameState::Menu;
                    }
                    2 => {
                        self.reset();
                        self.game_state = GameState::Playing;
                    },
                    _ => {}
                }
            },
            GameState::Pause => {
                match self.pause.as_ref().unwrap().mouse_button_down_event(button, x, y,
                    self.fabien.get_camera(), self.screen_size)
                {
                    1 => self.game_state = GameState::Playing,
                    2 => {
                        self.fabien.set_health(0);
                        self.game_state = GameState::Playing;
                    }

                    _ => {}
                }
            }
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        match self.game_state {
            GameState::Menu => {
                self.menu.mouse_motion_event(ctx, x, y);
            },
            GameState::Playing => {},
            GameState::GameOver => {
                self.game_over.as_mut().unwrap().mouse_motion_event(ctx, x, y);
            },
            GameState::Pause => {
                self.pause.as_mut().unwrap().mouse_motion_event(ctx, x, y, self.fabien.get_camera(), self.screen_size);
            }
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32)  {
        self.screen_size = (width, height);
        self.fabien.resize_event(width, height);
        match self.game_state {
            GameState::Menu => {
                self.menu.resize_event(ctx, width, height);
                ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
            },
            GameState::Playing => {},
            GameState::GameOver => {
                ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
                self.game_over.as_mut().unwrap().resize_event(ctx, width, height);
                self.menu.resize_event(ctx, width, height);
            },
            GameState::Pause => {
                self.pause.as_mut().unwrap().resize_event(ctx, self.fabien.get_camera());
            }
        }

    }
}

fn main() -> GameResult {
    dotenv().ok();
    let mut cb = ggez::ContextBuilder::new("B-Hunt", "Elzapat");
    cb = cb.window_setup(ggez::conf::WindowSetup {
        title: "B-Hunt".to_string(),
        icon: "/Fabien/Fabien_front_0.png".to_string(),
        vsync: true,
        ..Default::default()
    });
    cb = cb.window_mode(ggez::conf::WindowMode {
        // maximized: true,
        resizable: true,
        fullscreen_type: ggez::conf::FullscreenType::True,
        min_width: 800.0,
        min_height: 600.0,
        ..Default::default()
    });
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        cb = cb.add_resource_path(&path);
    }

    let (mut ctx, mut event_loop) = cb.build()?;
    let (width, height) = ggez::graphics::size(&mut ctx);
    println!("{:?}", (width, height));

    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    let state = &mut MainState::new(&mut ctx, width, height)?;
    event::run(&mut ctx, &mut event_loop, state)
}
