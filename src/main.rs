use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

mod map; use map::*;
mod fabien; use fabien::*;

enum GameState {
    Menu,
    Playing,
    GameOver,
}

struct MainState {
    map: Map,
}

impl MainState {
    fn new (ctx: &mut Context) -> GameResult<MainState> {
        let window_size = graphics::size(&ctx);
        let map = Map::new(ctx, window_size.0 as u32, window_size.1 as u32)?;
    
        let s = MainState {
            map: map,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut ggez::Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.map.draw(ctx)?;

        graphics::present(&mut ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("B-Hunt", "Elzapat");
    let (mut ctx, mut event_loop) = cb.build()?;

    graphics::set_window_icon(&mut ctx, Some("/tree-1.png"))?;
    graphics::set_window_title(&mut ctx, "B-Hunt");
    graphics::set_screen_coordinates(&mut ctx, graphics::Rect::new(
            10.0, 10.0, 100.0, 100.0))?;

    let state = &mut MainState::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, state)
}
