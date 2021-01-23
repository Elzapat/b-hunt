use rand::Rng;

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
