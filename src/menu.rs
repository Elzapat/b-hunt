use ggez::{
    GameResult, Context,
    graphics::Color,
    nalgebra::Point2,
    input::mouse::MouseButton
};
use crate::utils::loading_screen;
use crate::text::Text;
use crate::button::Button;
use std::collections::HashMap;
use serde::Deserialize;
use urlencoding::encode;
use std:: { fs, env };

#[derive(Deserialize, Debug)]
struct ApiResponse {
    response: Response
}

#[derive(Deserialize, Debug)]
struct Response {
    success: String,
    responses: Vec<FetchResponse>
}

#[derive(Deserialize, Debug)]
struct FetchResponse {
    success: String,
    data: String
}

enum MenuState {
    Main,
    Stats,
    Settings
}

#[derive(PartialEq, Eq, Hash)]
enum ButtonType {
    Back,
    Play,
    Stats,
    Settings,
    Quit
}

pub struct Menu {
    state: MenuState,
    texts: HashMap<String, Text>,
    buttons: HashMap<ButtonType, Button>
}

const BUTTON_WIDTH: f32 = 400.0;
const BUTTON_HEIGHT: f32 = 100.0;
const SPACING: f32 = 40.0;

impl Menu {
    pub fn new(ctx: &mut Context, screen_size: (f32, f32)) -> GameResult<Menu> {
        let font_path = "/Fonts/arial_narrow_7.ttf".to_string();

        let mut title_text = Text::new(ctx, String::from("B-Hunt"), font_path.clone(), 200.0, Color::new(1.0, 1.0, 1.0, 1.0))?;
        title_text.set_pos(Point2::new(screen_size.0 / 2.0 - title_text.width(ctx) / 2.0,
                screen_size.1 / 3.5 - title_text.height(ctx) / 2.0));

        let mut stats_text = Text::new(ctx, String::from(""), font_path.clone(), 40.0, Color::new(1.0, 1.0, 1.0, 1.0))?;
        stats_text.set_pos(Point2::new(100.0, 100.0));

        let color_not_hover = Color::from_rgb(255, 255, 255);
        let color_hover = Color::from_rgb(160, 160, 160);

        let play_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 2.0 - BUTTON_WIDTH - SPACING,
            screen_size.1 / 1.75 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 10.0, "Jouer".to_string())?;
        let stats_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 4.0 - BUTTON_WIDTH - SPACING,
            screen_size.1 / 1.3 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 10.0,  "Statistiques".to_string())?;
        let back_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 2.0 - BUTTON_WIDTH / 2.0,
            screen_size.1 / 1.3 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 10.0, "Retour".to_string())?;
        let quit_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 2.0 + SPACING,
            screen_size.1 / 1.3 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 10.0, "Quitter".to_string())?;
        let set_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, screen_size.0 / 2.0 + SPACING,
            screen_size.1 / 1.75 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 10.0, "Options".to_string())?;

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::Play, play_button);
        buttons.insert(ButtonType::Stats, stats_button);
        buttons.insert(ButtonType::Back, back_button);
        buttons.insert(ButtonType::Quit, quit_button);
        buttons.insert(ButtonType::Settings, set_button);

        let mut texts = HashMap::new();
        texts.insert("title".to_string(), title_text);
        texts.insert("stats".to_string(), stats_text);

        let menu = Menu {
            state: MenuState::Main,
            texts: texts,
            buttons: buttons
        };

        Ok(menu)
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        match self.state {
            MenuState::Main => {
                self.texts[&"title".to_string()].draw(ctx)?; 
                self.buttons[&ButtonType::Play].draw(ctx)?;
                self.buttons[&ButtonType::Stats].draw(ctx)?;
                self.buttons[&ButtonType::Quit].draw(ctx)?;
                self.buttons[&ButtonType::Settings].draw(ctx)?;
            },
            MenuState::Stats => {
                self.buttons[&ButtonType::Back].draw(ctx)?;
                self.texts[&"stats".to_string()].draw(ctx)?;
            },
            MenuState::Settings => {
                self.buttons[&ButtonType::Back].draw(ctx)?;
            }
        }

        Ok(())
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        for (_, button) in self.buttons.iter_mut() {
            button.mouse_motion_event(ctx, x, y);
        }
    }

    pub fn mouse_button_down_event(&mut self, ctx: &mut Context, x: f32, y: f32,
        mouse_button: MouseButton, screen_size: (f32, f32)) -> u8
    {
        if let MouseButton::Left = mouse_button {
            for (which, button) in self.buttons.iter() {
                if button.contains(x, y) {
                    match self.state {
                        MenuState::Main => {
                            match which {
                                ButtonType::Play => {
                                    self.texts.get_mut(&"stats".to_string()).unwrap().set_string(String::from(""));
                                    return 1;
                                },
                                ButtonType::Stats => {
                                    if self.texts[&"stats".to_string()].contents() == "" {
                                        loading_screen(ctx, screen_size);
                                        self.get_stats();
                                    }
                                    self.state = MenuState::Stats;
                                    break;
                                },
                                ButtonType::Quit => {
                                    ggez::event::quit(ctx);
                                },
                                ButtonType::Settings => {
                                    self.state = MenuState::Settings;
                                }
                                _ => {}
                            }
                        },
                        MenuState::Stats | MenuState::Settings => {
                            match which {
                                ButtonType::Back => {
                                    self.state = MenuState::Main;
                                    break;
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        0
    }

    fn get_stats(&mut self) {
        let mut error_message = String::from("");
        // Get the user info in the .gj_credentials file
        let mut user_info = String::from("");
        match fs::read_to_string(".gj-credentials") {
            Ok(info) => user_info = info,
            Err(e) => error_message = format!("Erreur : {}", e)
        }
        let user_info: Vec<&str> = user_info.split('\n').collect();
        let mut username = String::from("");
        let mut user_token = String::from("");
        if user_info.len() < 2 {
            error_message = "Vous n'avez pas lancé le jeu\navec le client GameJolt donc vous\n\
                        ne pouvez pas voir vos statistiqes.".to_string();
        } else {
            username = user_info[1].to_string();
            user_token = user_info[2].to_string();
        }

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

        let keys: [&str; 6] = ["bertrand_killed", "shots", "powerups_activated",
                               "hits_taken", "time_played", "games_played"];

        let mut api_url = format!("https://api.gamejolt.com/api/game/v1_2/batch?game_id={}", game_id);

        for key in keys.iter() {
            let mut url = format!("/data-store/?game_id={}&key={}&username={}&user_token={}",
                                    game_id, key, username, user_token);
            let mut hasher = sha1::Sha1::new();
            hasher.update(format!("{}{}", url, private_key).as_bytes());
            let signature = hasher.digest().to_string();
            url = format!("{}&signature={}", url, signature);
            url = encode(&url); 

            api_url = format!("{}&requests[]={}", api_url, url);
        }

        let mut hasher = sha1::Sha1::new();
        hasher.update(format!("{}&parallel=true{}", api_url, private_key).as_bytes());
        let signature = hasher.digest().to_string();
        api_url = format!("{}&parallel=true&signature={}", api_url, signature);

        let res;
        match reqwest::blocking::get(&api_url) {
            Ok(r) => res = r,
            Err(e) => { self.texts.get_mut(&"stats".to_string()).unwrap().set_string(format!("Erreur : {}", e)); return; }
        }

        let response: ApiResponse;
        match res.json() {
            Ok(r) => response = r,
            Err(e) => { self.texts.get_mut(&"stats".to_string()).unwrap().set_string(format!("Erreur : {}", e)); return; }
        }
        
        let mut stats;
        if error_message != "" {
            stats = error_message;
        } else {
            stats = format!("Statistiques pour {} :\n\n", username);
            for (i, data) in response.response.responses.iter().enumerate() {
                if i >= keys.len() { break; }

                let stat_name = match keys[i] {
                    "bertrand_killed" => "Nombre d'ennemies tués",
                    "shots" => "Nombre de coup tirés",
                    "powerups_activated" => "Nombre de powerups activés",
                    "hits_taken" => "Nombre de coups pris",
                    "time_played" => "Temps joué (en secondes)",
                    "games_played" => "Nombre de parties jouées",
                    _ => unreachable!()
                };

                if data.success == "true" {
                    stats = format!("{}{} : {}\n", stats, stat_name, data.data);
                }
            }
        }

        self.texts.get_mut(&"stats".to_string()).unwrap().set_string(stats);
    }

    pub fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        let title_text = self.texts.get_mut(&"title".to_string()).unwrap();
        title_text.set_pos(Point2::new(width / 2.0 - title_text.width(ctx) / 2.0,
                height / 3.5 - title_text.height(ctx) / 2.0));

        let stats_text = self.texts.get_mut(&"stats".to_string()).unwrap();
        stats_text.set_pos(Point2::new(100.0, 100.0));

        let play_button = self.buttons.get_mut(&ButtonType::Play).unwrap();
        play_button.set_pos(ctx, width / 2.0 - BUTTON_WIDTH - SPACING, height / 1.75 - BUTTON_HEIGHT / 2.0);

        let stats_button = self.buttons.get_mut(&ButtonType::Stats).unwrap();
        stats_button.set_pos(ctx, width / 2.0 - BUTTON_WIDTH - SPACING, height / 1.3 - BUTTON_HEIGHT / 2.0);

        let back_button = self.buttons.get_mut(&ButtonType::Back).unwrap();
        back_button.set_pos(ctx, width / 2.0 - BUTTON_WIDTH / 2.0, height / 1.3 - BUTTON_HEIGHT / 2.0);

        let set_button = self.buttons.get_mut(&ButtonType::Settings).unwrap();
        set_button.set_pos(ctx, width / 2.0 + SPACING, height / 1.75 - BUTTON_HEIGHT / 2.0);

        let quit_button = self.buttons.get_mut(&ButtonType::Quit).unwrap();
        quit_button.set_pos(ctx, width / 2.0 + SPACING, height / 1.3 - BUTTON_HEIGHT / 2.0);
    }
}
