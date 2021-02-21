use ggez::{
    graphics, GameResult, Context,
    graphics::{ Text, TextFragment, Font },
    nalgebra::Point2
};
use crate::utils::Stats;
use std::{ fs, env };
use serde::Deserialize;
use serde_json::Value;
use urlencoding::encode;

#[derive(Deserialize, Debug)]
struct Score{
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
    game_over_text: Text,
    leaderboard_text: Text,
    current_score_text: Text,
    press_a_button_text: Text
}

impl GameOver {
    pub fn new(ctx: &mut Context, cur_score: u32, stats: Stats, screen_size: (f32, f32)) -> GameResult<GameOver> {
        // Font for the text
        let font = Font::new(ctx, "/Fonts/arial_narrow_7.ttf").unwrap();

        graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, screen_size.0, screen_size.1))?;
        // Display a loading text, because accessing the api takes a bit of time
        let shade_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, screen_size.0, screen_size.1),
            graphics::Color::new(0.0, 0.0, 0.0, 0.9)
        ).unwrap();
        graphics::draw(ctx, &shade_rect, graphics::DrawParam::default())?;
        let loading_fragment = TextFragment::new("Chargement...")
            .font(font)
            .scale(graphics::Scale::uniform(120.0));
        let loading_text = Text::new(loading_fragment);
        let dest = Point2::new(
            screen_size.0 / 2.0 - loading_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 2.0 - loading_text.height(ctx) as f32 / 2.0
        );
        graphics::draw(ctx, &loading_text, graphics::DrawParam::default().dest(dest))?;
        graphics::present(ctx)?;

        let mut error_message = String::from("");
        // Get the user info in the .gj_credentials file
        let user_file = fs::read_to_string(".gj-credentials");
        let mut user_info = String::from("");
        match user_file {
            Ok(info) => user_info = info,
            Err(e) => error_message = format!("Erreur : {}", e)
        }
        let user_info: Vec<&str> = user_info.split('\n').collect();
        let username = user_info[1].to_string();
        let user_token = user_info[2].to_string();

        // Get the useful infos to access the GameJolt API
        let mut game_id = String::from(""); let mut private_key = String::from("");
        match env::var("GAME_ID") {
            Ok(id) => game_id = id,
            Err(_) => error_message = String::from("Variable manquante (GAME_ID)")
        }
        match env::var("PRIVATE_KEY") {
            Ok(key) => private_key = key,
            Err(_) => error_message = String::from("Variable manquante (PRIVATE_KEY)")
        }
        println!("error_message: {}", error_message);

        // Get the leaderboard with the database
        let leaderboard = set_leaderboard(cur_score, username.clone(),
                            user_token.clone(), game_id.clone(), private_key.clone());
        // Record the player's stats for this game
        let stats_status = record_stats(stats, username.clone(),
                            user_token.clone(), game_id.clone(), private_key.clone());
        println!("leaderboard: {}", leaderboard);
        println!("stats_status: {}", stats_status);


        let game_over_fragment = TextFragment::new("Game Over")
            .font(font)
            .scale(graphics::Scale::uniform(150.0));
        let game_over_text = Text::new(game_over_fragment);

        let leaderboard_fragment = TextFragment::new(leaderboard)
            .font(font)
            .scale(graphics::Scale::uniform(50.0));
        let leaderboard_text = Text::new(leaderboard_fragment);

        let current_score_fragment = TextFragment::new(format!("Votre score pour cette partie : {}\n{}",
                                                                    cur_score, stats_status))
            .font(font)
            .scale(graphics::Scale::uniform(30.0));
        let current_score_text = Text::new(current_score_fragment);

        let press_a_button_text_fragment = TextFragment::new(
            "Cliquez n'importe où pour rejouer...")
            .font(font)
            .scale(graphics::Scale::uniform(30.0));
        let press_a_button_text = Text::new(press_a_button_text_fragment);

        let game_over = GameOver {
            game_over_text: game_over_text,
            leaderboard_text: leaderboard_text,
            current_score_text: current_score_text,
            press_a_button_text: press_a_button_text
        };

        Ok(game_over)
    }


    pub fn update(&self) {

    }

    pub fn draw(&self, ctx: &mut Context, screen_size: (f32, f32)) -> GameResult {
        let dest = Point2::new(
            screen_size.0 / 2.0 - self.game_over_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 5.0 - self.game_over_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::default().dest(dest);
        graphics::draw(ctx, &self.game_over_text, draw_param)?;

        let dest = Point2::new(
            screen_size.0 / 2.0 - self.leaderboard_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 1.8 - self.leaderboard_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::default().dest(dest);
        graphics::draw(ctx, &self.leaderboard_text, draw_param)?;

        let dest = Point2::new(
            screen_size.0 / 2.0 - self.current_score_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 1.2 - self.current_score_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::new().dest(dest);
        graphics::draw(ctx, &self.current_score_text, draw_param)?;

        let dest = Point2::new(
            screen_size.0 / 2.0 - self.press_a_button_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 1.1 - self.press_a_button_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::new().dest(dest);
        graphics::draw(ctx, &self.press_a_button_text, draw_param)?;

        Ok(())
    }
}

fn record_stats(stats: Stats, username: String, user_token: String,
            game_id: String, private_key: String) -> String {
    // Check if the user has stats in the GameJolt API, if not create it
    let api_url = "https://api.gamejolt.com/api/game/v1_2/data-store/?";

    let keys: [&str; 6] = ["bertrand_killed", "shots", "powerups_activated",
                           "hits_taken", "time_played", "games_played"];

    for key in keys.iter() {
        let mut url = format!("{}game_id={}&key={}&username={}&user_token={}",
                            api_url, game_id, key, username, user_token); 
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", url, private_key).as_bytes());
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
                            url, game_id, key, username, user_token);
            let mut hasher = sha1::Sha1::new();
            hasher.update(format!("{}{}", url, private_key).as_bytes());
            let signature = hasher.digest().to_string();
            url = format!("{}&signature={}", url, signature);
            match reqwest::blocking::get(&url) {
                Ok(_) => {},
                Err(e) => return format!("Erreur stats : {}", e)
            }
        }
    }

    let api_url = format!("https://api.gamejolt.com/api/game/v1_2/batch?game_id={}", game_id);
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
                                    game_id, key, value);
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", global_url, private_key).as_bytes());
        let signature = hasher.digest().to_string();
        global_url = format!("{}&signature={}", global_url, signature);
        global_url = encode(&global_url); 

        let mut user_url = format!("/data-store/update/?game_id={}&key={}&username={}&user_token={}&operation=add&value={}",
                                    game_id, key, username, user_token, value);
        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}{}", user_url, private_key).as_bytes());
        let signature = hasher.digest().to_string();
        user_url = format!("{}&signature={}", user_url, signature);
        user_url = encode(&user_url);

        global_api_url = format!("{}&requests[]={}", global_api_url, global_url);
        user_api_url = format!("{}&requests[]={}", user_api_url, user_url);
    }

    let mut hasher = sha1::Sha1::new();
    hasher.update(format!("{}{}", global_api_url, private_key).as_bytes());
    let signature = hasher.digest().to_string();
    global_api_url = format!("{}&signature={}", global_api_url, signature);

    hasher = sha1::Sha1::new();
    hasher.update(format!("{}{}", user_api_url, private_key).as_bytes());
    let signature = hasher.digest().to_string();
    user_api_url = format!("{}&signature={}", user_api_url, signature);

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

fn set_leaderboard(cur_score: u32, username: String, user_token: String,
            game_id: String, private_key: String) -> String {

    let mut api_url = "https://api.gamejolt.com/api/game/v1_2/scores/add/?".to_string();
    let table_id = "594910";
    let mut leaderboard = String::from("");
    let mut hasher = sha1::Sha1::new();

    // Record the score using the GameJolt API
    api_url = format!("{}game_id={}&username={}&user_token={}&score={}&sort={}&table_id={}",
                        api_url, game_id, username, user_token, cur_score, cur_score, table_id);
    hasher.update(format!("{}{}", api_url, private_key).as_bytes());
    let signature = hasher.digest().to_string();
    api_url = format!("{}&signature={}", api_url, signature);
    match reqwest::blocking::get(&api_url) {
        Ok(_) => {},
        Err(e) => return format!("Erreur : {}", e)
    }

    // Get the highest scores from the GameJolt API
    hasher = sha1::Sha1::new();
    let mut api_url = "https://api.gamejolt.com/api/game/v1_2/scores/?game_id=587107&limit=5&table_id=594910".to_string();
    hasher.update(format!("{}{}", api_url, private_key).as_bytes());
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
