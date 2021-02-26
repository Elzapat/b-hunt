use rand::Rng;
use ggez::{
    graphics::{ Text, TextFragment, Font, Scale, Rect },
    graphics, Context,
    nalgebra::Point2
};
use std::{ env, fs };

#[derive(Debug, Copy, Clone)]
pub struct Stats {
    pub bertrand_killed: u64,
    pub shots: u64,
    pub powerups_activated: u64,
    pub hits_taken: u64,
    pub time_played: u64
}

#[derive(Clone)]
pub struct Credentials {
    pub username: String,
    pub user_token: String,
    pub game_id: String,
    pub private_key: String
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

pub fn loading_screen(ctx: &mut Context, screen_size: (f32, f32)) {
    ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, screen_size.0, screen_size.1)).unwrap();
    let shade_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(0.0, 0.0, screen_size.0, screen_size.1),
        graphics::Color::new(0.0, 0.0, 0.0, 0.9)
    ).unwrap();
    graphics::draw(ctx, &shade_rect, graphics::DrawParam::default()).unwrap();

    let loading_text = Text::new(
        TextFragment::new("Chargement...")
            .font(Font::new(ctx, "/Fonts/arial_narrow_7.ttf").unwrap())
            .scale(Scale::uniform(120.0))
    );
    let dest = Point2::new(
        screen_size.0 / 2.0 - loading_text.width(ctx) as f32 / 2.0,
        screen_size.1 / 2.0 - loading_text.height(ctx) as f32 / 2.0
    );
    graphics::draw(ctx, &loading_text, graphics::DrawParam::default().dest(dest)).unwrap();
    graphics::present(ctx).unwrap();
}

pub fn get_credentials() -> Result<Credentials, String> {

    // Get the user info in the .gj_credentials file
    let user_info;
    match fs::read_to_string(".gj-credentials") {
        Ok(info) => user_info = info,
        Err(e) => return Err(format!("Erreur : {}", e))
    }
    let user_info: Vec<&str> = user_info.split('\n').collect();
    let username;
    let user_token;
    if user_info.len() < 3 {
        let error_message = "Vous n'avez pas lancé le jeu avec le client GameJolt donc\n\
            vos statistiques et votre score ne peuvent pas être enregistrés".to_string();
        return Err(error_message);
    }

    username = user_info[1].to_string();
    user_token = user_info[2].to_string();

    // Get the useful infos to access the GameJolt API
    let game_id; let private_key;
    match env::var("GAME_ID") {
        Ok(id) => game_id = id,
        Err(_) => return Err(String::from("Erreur : variable manquante (GAME_ID)"))
    }
    match env::var("PRIVATE_KEY") {
        Ok(key) => private_key = key,
        Err(_) => return Err(String::from("Erreur : variable manquante (PRIVATE_KEY)"))
    }

    let cred = Credentials {
        username: username,
        user_token: user_token,
        game_id: game_id,
        private_key: private_key
    };

    Ok(cred)
}
