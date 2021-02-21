use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub bertrand_killed: u64,
    pub shots: u64,
    pub powerups_activated: u64,
    pub hits_taken: u64,
    pub time_played: u64
}

#[derive(PartialEq)]
pub enum Movement {
    Up,
    Down,
    Right,
    Left,
}

pub fn rand(max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let random_f: f64 = rng.gen();
    (random_f * max as f64).round() as f32 
}
