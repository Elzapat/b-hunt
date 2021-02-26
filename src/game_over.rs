use ggez::{
    GameResult, Context,
    graphics::Color,
    nalgebra::Point2,
    input::mouse::MouseButton
};
use crate::utils::*;
use crate::button::Button;
use crate::text::Text;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
use urlencoding::encode;

#[derive(Deserialize, Debug)]
struct Score {
    score: String,
    sort: String,
    user: String
}

#[derive(Deserialize, Debug)]
struct Response {
    success: String,
    scores: Vec<Score>
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    response: Response
}

pub struct GameOver {
    texts: HashMap<String, Text>,
    buttons: HashMap<String, Button>
}

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 50.0;

impl GameOver {
    pub fn new(ctx: &mut Context, cur_score: u32, stats: Stats, screen_size: (f32, f32)) -> GameResult<GameOver> {
        loading_screen(ctx, screen_size);

        let leaderboard; let stats_status;
        match get_credentials() {
            Ok(cred) => {
                leaderboard = set_leaderboard(cur_score, cred.clone());
                stats_status = record_stats(stats, cred.clone());
            },
            Err(e) => {
                leaderboard = e.clone();
                stats_status = e.clone();
            }
        }

        let font_path = "/Fonts/arial_narrow_7.ttf".to_string();

        let mut game_over_text = Text::new(ctx, String::from("Game Over"), font_path.clone(), 150.0, Color::new(1.0, 1.0, 1.0, 1.0))?;
        game_over_text.set_pos(Point2::new(screen_size.0 / 2.0 - game_over_text.width(ctx) / 2.0,
                screen_size.1 / 5.0 - game_over_text.height(ctx) / 2.0));

        let mut leaderboard_text = Text::new(ctx, String::from(leaderboard), font_path.clone(), 50.0, Color::new(1.0, 1.0, 1.0, 1.0))?;
        leaderboard_text.set_pos(Point2::new(screen_size.0 / 2.0 - leaderboard_text.width(ctx) / 2.0,
                screen_size.1 / 2.4 - game_over_text.height(ctx) / 2.0));

        let mut score_text = Text::new(ctx, String::from(format!("Score : {}\n{}", cur_score, stats_status)),
            font_path.clone(), 30.0, Color::new(1.0, 1.0, 1.0, 1.0))?;
        score_text.set_pos(Point2::new(screen_size.0 / 2.0 - score_text.width(ctx) / 2.0,
                screen_size.1 / 1.3 - score_text.height(ctx) / 2.0));

        let color_not_hover = Color::from_rgb(255, 255, 255);
        let color_hover = Color::from_rgb(160, 160, 160);

        let menu_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 4.0 - BUTTON_WIDTH / 2.0,
            screen_size.1 / 1.1 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 5.0, "Menu".to_string())?;
        let replay_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, (3.0 * screen_size.0) / 4.0 - BUTTON_WIDTH / 2.0,
            screen_size.1 / 1.1 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 5.0, "Rejouer".to_string())?;

        let mut buttons = HashMap::new();
        buttons.insert("menu".to_string(), menu_button);
        buttons.insert("replay".to_string(), replay_button);

        let mut texts = HashMap::new();
        texts.insert("game_over".to_string(), game_over_text);
        texts.insert("leaderboard".to_string(), leaderboard_text);
        texts.insert("score".to_string(), score_text);

        let game_over = GameOver {
            texts: texts,
            buttons: buttons
        };

        Ok(game_over)
    }


    pub fn update(&self) {

    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for (_, text) in self.texts.iter() {
            text.draw(ctx)?;
        }

        for (_, button) in self.buttons.iter() {
            button.draw(ctx)?;
        }

        Ok(())
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        for (_, button) in self.buttons.iter_mut() {
            button.mouse_motion_event(ctx, x, y);
        }
    }

    pub fn mouse_button_down_event(&self, mouse_button: MouseButton, x: f32, y: f32) -> u8 {
        if let MouseButton::Left = mouse_button {
            for (which, button) in self.buttons.iter() {
                if button.contains(x, y) {
                    match &which[..] {
                        "menu" => return 1,
                        "replay" => return 2,
                        _ => unreachable!()
                    }
                }
            }
        }
        0
    }

    pub fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let game_over_text = self.texts.get_mut(&"game_over".to_string()).unwrap();
        let (t_w, t_h) = (game_over_text.width(ctx), game_over_text.height(ctx));
        game_over_text.set_pos(Point2::new(width / 2.0 - t_w / 2.0,
                height / 5.0 - t_h / 2.0));

        let leaderboard_text = self.texts.get_mut(&"leaderboard".to_string()).unwrap();
        let (t_w, t_h) = (leaderboard_text.width(ctx), leaderboard_text.height(ctx));
        leaderboard_text.set_pos(Point2::new(width / 2.0 - t_w / 2.0,
                height / 2.4 - t_h / 2.0));

        let score_text = self.texts.get_mut(&"score".to_string()).unwrap();
        let (t_w, t_h) = (score_text.width(ctx), score_text.height(ctx));
        score_text.set_pos(Point2::new(width / 2.0 - t_w / 2.0,
                height / 1.3 - t_h / 2.0));

        let menu_button = self.buttons.get_mut(&"menu".to_string()).unwrap();
        menu_button.set_pos(ctx, width / 4.0 - BUTTON_WIDTH / 2.0, height / 1.1 - BUTTON_HEIGHT / 2.0);

        let replay_button = self.buttons.get_mut(&"replay".to_string()).unwrap();
        replay_button.set_pos(ctx, (3.0 * width) / 4.0 - BUTTON_WIDTH / 2.0, height / 1.1 - BUTTON_HEIGHT / 2.0);
    }
}

fn record_stats(stats: Stats, cred: Credentials) -> String {
    // Check if the user has stats in the GameJolt API, if not create it
    let api_url = "https://api.gamejolt.com/api/game/v1_2/data-store/?";

    let keys: [&str; 6] = ["bertrand_killed", "shots", "powerups_activated",
                           "hits_taken", "time_played", "games_played"];

    for key in keys.iter() {
        let mut url = format!("{}game_id={}&key={}&username={}&user_token={}",
                            api_url, cred.game_id, key, cred.username, cred.user_token); 
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", url, cred.private_key).as_bytes());
        let signature = hasher.digest().to_string();
        url = format!("{}&signature={}", url, signature);
        let res;
        match reqwest::blocking::get(&url) {
            Ok(r) => res = r,
            Err(e) => return format!("Erreur stats : {}", e)
        }
        let text_res;
        match res.text() {
            Ok(t) => text_res = t,
            Err(e) => return format!("Erreur stats : {}", e)
        }
        let res: Value;
        match serde_json::from_str(&text_res) {
            Ok(v) => res = v,
            Err(e) => return format!("Erreur stats : {}", e)
        }
        if res["response"]["success"] == "false" {
            let mut url = "https://api.gamejolt.com/api/game/v1_2/data-store/set/?".to_string();
            url = format!("{}game_id={}&key={}&data=0&username={}&user_token={}",
                            url, cred.game_id, key, cred.username, cred.user_token);
            let mut hasher = sha1::Sha1::new();
            hasher.update(format!("{}{}", url, cred.private_key).as_bytes());
            let signature = hasher.digest().to_string();
            url = format!("{}&signature={}", url, signature);
            match reqwest::blocking::get(&url) {
                Ok(_) => {},
                Err(e) => return format!("Erreur stats : {}", e)
            }
        }
    }

    let api_url = format!("https://api.gamejolt.com/api/game/v1_2/batch?game_id={}", cred.game_id);
    let mut global_api_url = api_url.clone();
    let mut user_api_url = api_url.clone();

    for key in keys.iter() {
        let value = match *key {
            "bertrand_killed" => stats.bertrand_killed,
            "games_played" => 1,
            "hits_taken" => stats.hits_taken,
            "powerups_activated" => stats.powerups_activated,
            "shots" => stats.shots,
            "time_played" => stats.time_played,
            _ => unreachable!()
        };
        let mut global_url = format!("/data-store/update/?game_id={}&key={}&operation=add&value={}",
                                    cred.game_id, key, value);
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", global_url, cred.private_key).as_bytes());
        let signature = hasher.digest().to_string();
        global_url = format!("{}&signature={}", global_url, signature);
        global_url = encode(&global_url); 

        let mut user_url = format!("/data-store/update/?game_id={}&key={}&username={}&user_token={}&operation=add&value={}",
                                    cred.game_id, key, cred.username, cred.user_token, value);
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", user_url, cred.private_key).as_bytes());
        let signature = hasher.digest().to_string();
        user_url = format!("{}&signature={}", user_url, signature);
        user_url = encode(&user_url);

        global_api_url = format!("{}&requests[]={}", global_api_url, global_url);
        user_api_url = format!("{}&requests[]={}", user_api_url, user_url);
    }

    let mut hasher = sha1::Sha1::new();
    hasher.update(format!("{}&parallel=true{}", global_api_url, cred.private_key).as_bytes());
    let signature = hasher.digest().to_string();
    global_api_url = format!("{}&parallel=true&signature={}", global_api_url, signature);

    hasher = sha1::Sha1::new();
    hasher.update(format!("{}&parallel=true{}", user_api_url, cred.private_key).as_bytes());
    let signature = hasher.digest().to_string();
    user_api_url = format!("{}&parallel=true&signature={}", user_api_url, signature);

    // println!("global: {}, user: {}", global_api_url, user_api_url);

    match reqwest::blocking::get(&global_api_url) {
        Ok(_) => {},
        Err(e) => return format!("Erreur stats : {}", e)
    }
    match reqwest::blocking::get(&user_api_url) {
        Ok(_) => {},
        Err(e) => return format!("Erreur stats : {}", e)
    }

    String::from("Vos statistiqes ont été enregistrées")
}

fn set_leaderboard(cur_score: u32, cred: Credentials) -> String {

    let mut api_url = "https://api.gamejolt.com/api/game/v1_2/scores/add/?".to_string();
    let table_id = "594910";
    let mut leaderboard = String::from("");
    let mut hasher = sha1::Sha1::new();

    // Record the score using the GameJolt API
    api_url = format!("{}game_id={}&username={}&user_token={}&score={}&sort={}&table_id={}",
                        api_url, cred.game_id, cred.username, cred.user_token, cur_score, cur_score, table_id);
    hasher.update(format!("{}{}", api_url, cred.private_key).as_bytes());
    let signature = hasher.digest().to_string();
    api_url = format!("{}&signature={}", api_url, signature);
    match reqwest::blocking::get(&api_url) {
        Ok(_) => {},
        Err(e) => return format!("Erreur : {}", e)
    }

    // Get the highest scores from the GameJolt API
    hasher = sha1::Sha1::new();
    let mut api_url = "https://api.gamejolt.com/api/game/v1_2/scores/?game_id=587107&limit=5&table_id=594910".to_string();
    hasher.update(format!("{}{}", api_url, cred.private_key).as_bytes());
    let signature = hasher.digest().to_string();
    api_url = format!("{}&signature={}", api_url, signature);
    let res;
    match reqwest::blocking::get(&api_url) {
        Ok(r) => res = r,
        Err(e) => return format!("Erreur : {}", e)
    }

    let response: ApiResponse;
    match res.json() {
        Ok(r) => response = r,
        Err(e) => return format!("Erreur : {}", e)
    }

    if response.response.success == "true" {
        for i in 0..response.response.scores.len() {
            leaderboard = format!("{}{}. {} : {}\n", leaderboard, i + 1,
                response.response.scores[i].user, response.response.scores[i].sort);
        }
    } else {
        return String::from("Erreur au chargement des scores.");
    }

    leaderboard
}
