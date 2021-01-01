use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::event::*;

mod map; use map::*;
mod fabien; use fabien::*;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct MainState {
    map: Map,
    fabien: Fabien,
    frames: u32
}

impl MainState {
    fn new (ctx: &mut Context, width: f32, height: f32) -> GameResult<MainState> {
        let map = Map::new(ctx, width, height)?;
        let fabien = Fabien::new(ctx, width, height)?;
        let s = MainState {
            map: map,
            fabien: fabien,
            frames: 0
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> GameResult {
        self.fabien.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut ggez::Context) -> GameResult {
        //println!("FPS: {}", ggez::timer::fps(ctx));
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.map.draw(ctx)?;
        self.fabien.draw(ctx, &self.frames)?;

        graphics::present(&mut ctx)?;

        self.frames = (self.frames + 1) % 40;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods, _repeat: bool) {
        self.fabien.key_down_event(keycode);
    }
    
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.fabien.key_up_event(keycode);
    }
}

pub fn main() -> ggez::GameResult {

    let mut cb = ggez::ContextBuilder::new("B-Hunt", "Elzapat");
    cb = cb.window_setup(ggez::conf::WindowSetup {
        title: "B-Hunt".to_string(),
        vsync: true,
        icon: "/Fabien_front_0.png".to_string(),
        ..Default::default()
    });
    let (width, height) = (800.0, 600.0);
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
