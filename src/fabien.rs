use ggez::graphics;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use std::collections::HashMap;

enum FabienState {
    Idle,
    Walking,
}

// Fabien is the player
pub struct Fabien {
    sprites: HashMap<String, graphics::Image>,
    state: FabienState,
    hitbox: Rect,
    ammos: u32,
    score: u32,
    health: u8,
}

impl Fabien {
    /*
    pub fn new() -> GameResult<Fabien> {
    }
    */
}
