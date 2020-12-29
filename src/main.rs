use ggez;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

mod map; use map::*;

struct MainState {
    map: Map,
}

impl MainState {
    fn new (ctx: &mut Context) -> GameResult<MainState> {
        let map = Map::new(ctx, 800, 600);
    
        let s = MainState {
            map: map,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        Ok(())
    }

    fn draw(&mut self, mut ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        self.map.draw(ctx)?;

        graphics::present(&mut ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
