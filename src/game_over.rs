use ggez::{
    graphics, GameResult, Context,
    graphics::{ Text, TextFragment, Font },
    nalgebra::Point2
};
use mysql::prelude::*;
use dotenv::dotenv;
use std::env;

struct Score {
    username: String,
    score: u32,
    os: String,
    game_version: String
}

pub struct GameOver {
    game_over_text: Text,
    leaderboard_text: Text,
    current_score_text: Text,
    press_a_button_text: Text
}

impl GameOver {
    pub fn new(ctx: &mut Context, cur_score: u32, username: String,
        os: String, game_version: String) -> GameResult<GameOver> {

        // Get the leaderboard with the database
        let leaderboard = set_leaderboard(username, cur_score, os, game_version);
        println!("{}", leaderboard);

        let font = Font::new(ctx, "/Fonts/arial_narrow_7.ttf").unwrap();

        let game_over_fragment = TextFragment::new("Game Over")
            .font(font)
            .scale(graphics::Scale::uniform(150.0));
        let game_over_text = Text::new(game_over_fragment);

        let leaderboard_fragment = TextFragment::new(leaderboard)
            .font(font)
            .scale(graphics::Scale::uniform(50.0));
        let leaderboard_text = Text::new(leaderboard_fragment);

        let current_score_fragment = TextFragment::new(format!("Votre score pour cette partie : {}", cur_score))
            .font(font)
            .scale(graphics::Scale::uniform(30.0));
        let current_score_text = Text::new(current_score_fragment);

        let press_a_button_text_fragment = TextFragment::new(
            "Appuyez sur n'importe quelle touche pour rejouer...")
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

fn set_leaderboard(username: String, cur_score: u32, os: String, game_version: String) -> String {
    dotenv().ok();
    let mut database_url = "mysql://bhunt_user:bhunt".to_owned();
    match env::var("PASSWORD") {
        Ok(pwd) => database_url += &pwd,
        Err(_) => return String::from("Variable manquante (PASSWORD)")
    }
    database_url += &"@82.65.14.54/bhunt".to_owned();
    let error_message = String::from("Erreur à l'accès à la base de données.");

    let pool; let mut conn; let mut scores;
    match mysql::Pool::new(database_url) {
        Ok(p) => pool = p,
        Err(e) => { println!("{:?}", e); return error_message; }
    }
    match pool.get_conn() {
        Ok(c) => conn = c,
        Err(e) => { println!("{:?}", e); return error_message; }
    }
    let res = conn.query_map(
        format!("SELECT username, score, os, game_version
                 FROM scores WHERE username = \"{}\";", username),
        |(username, score, os, game_version)| {
            Score { username, score, os, game_version }
        }
    );
    match res {
        Ok(s) => scores = s,
        Err(e) => { println!("{:?}", e); return error_message; } 
    }

    if scores.is_empty() {
        let res = conn.query_drop(
            format!("INSERT INTO scores (username, score, os, game_version)
                     VALUES (\"{}\", {}, \"{}\", \"{}\");", username, cur_score, os, game_version)
        );
        match res {
            Ok(_) => {},
            Err(e) => { println!("{:?}", e); return error_message; } 
        }
    } else if cur_score > scores[0].score {
        let res = conn.query_drop(
            format!("UPDATE scores SET score = {}
                     WHERE username = \"{}\";", cur_score, username)
        );
        match res {
            Ok(_) => {},
            Err(e) => { println!("{:?}", e); return error_message; }
        }
    }

    let mut leaderboard = String::from("");
    const NB_POSITIONS: usize = 5;
    let res = conn.query_map(
        format!("SELECT username, score, os, game_version
            FROM scores ORDER BY score DESC LIMIT {};", NB_POSITIONS),
        |(username, score, os, game_version)| {
            Score { username, score, os, game_version } 
        }
    );
    match res {
        Ok(s) => scores = s,
        Err(e) => { println!("{:?}", e); return error_message; }
    }

    for i in 1..=NB_POSITIONS {
        if i - 1 >= scores.len() {
            leaderboard.push_str(&format!("{}.\n", i)[..]);
        } else {
            leaderboard.push_str(&format!("{}. {} : {}\n", i, scores[i - 1].username, scores[i - 1].score)[..]);
        }
    }

    leaderboard
}
