use ggez::{
    Context, GameResult,
    graphics,
    graphics::{ Color, Rect },
    nalgebra::{ Vector2, Point2 },
    input::mouse::MouseButton
};
use crate::button::Button;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash)]
enum ButtonType {
    BackToGame,
    GiveUp
}

pub struct Pause {
    pause_image: graphics::Image,
    buttons: HashMap<ButtonType, Button>
}

const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 25.0;

impl Pause {
    pub fn new(ctx: &mut Context, camera: Rect) -> GameResult<Pause> {
        // graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, camera.w, camera.h))?;
        let pause_image = graphics::Image::new(ctx, "/pause.png")?;

        let color_not_hover = Color::from_rgb(255, 255, 255);
        let color_hover = Color::from_rgb(160, 160, 160);

        let mut back_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, camera.x + camera.w / 2.0 - BUTTON_WIDTH / 2.0,
            camera.y + camera.h / 1.75 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 2.0, String::from("Retour au jeu"))?;
        back_button.set_text_scale(128.0);

        let mut quit_button = Button::new(ctx, BUTTON_WIDTH, BUTTON_HEIGHT, camera.x + camera.w / 2.0 - BUTTON_WIDTH / 2.0,
            camera.y + camera.h / 1.3 - BUTTON_HEIGHT / 2.0, color_not_hover, color_hover, 2.0, String::from("Abandonner"))?;
        quit_button.set_text_scale(128.0);

        let mut buttons = HashMap::new();
        buttons.insert(ButtonType::BackToGame, back_button);
        buttons.insert(ButtonType::GiveUp, quit_button);

        let pause = Pause {
            pause_image: pause_image,
            buttons: buttons
        };
        Ok(pause)
    }

    pub fn draw(&self, ctx: &mut Context, camera: Rect) -> GameResult {
        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                camera.x + camera.w / 2.0 - (self.pause_image.width() as f32 * 1.5)  / 2.0, 
                camera.y + camera.h / 3.5 - (self.pause_image.height() as f32 * 1.5) / 2.0
            )).scale(Vector2::new(1.5, 1.5));
        graphics::draw(ctx, &self.pause_image, param)?;

        // Drawing the buttons and texts "manually" because the screen is zoomed in so it's a pain
        // (it ended up not being too much of a pain, but it was a pain to end up in this
        // non-painful way).
        const SCALE: f32 = 0.1;
        for (_, button) in self.buttons.iter() {
            graphics::draw(ctx, button.get_border(), (button.get_pos(),))?;

            let param = graphics::DrawParam::new()
                .dest(button.get_text().get_pos())
                .scale(Vector2::new(SCALE, SCALE));
            graphics::draw(ctx, button.get_text().get_ggez_text(), param)?;
        } 

        Ok(())
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, camera: Rect, screen_size: (f32, f32)) {
        // Have to apply the screen / zoom ratio the the mouse pos because they're
        // the window mouse pos
        let (x_ratio, y_ratio) = (screen_size.0 / camera.w, screen_size.1 / camera.h);
        for (_, button) in self.buttons.iter_mut() {
            button.mouse_motion_event(ctx, camera.x + x / x_ratio, camera.y + y / y_ratio);
        }
    }

    pub fn mouse_button_down_event(&self, mouse_button: MouseButton, x: f32, y: f32,
        camera: Rect, screen_size: (f32, f32)) -> u8
    {
        // Same deal as above 
        if let MouseButton::Left = mouse_button {
            let (x_ratio, y_ratio) = (screen_size.0 / camera.w, screen_size.1 / camera.h);
            for (which, button) in self.buttons.iter() {
                if button.contains(camera.x + x / x_ratio, camera.y + y / y_ratio) {
                    match which {
                        ButtonType::BackToGame => return 1,
                        ButtonType::GiveUp => return 2
                    }
                }
            }
        }
        0
    }

    pub fn resize_event(&mut self, ctx: &mut Context, camera: Rect) {
        let back_button = self.buttons.get_mut(&ButtonType::BackToGame).unwrap(); 
        back_button.set_pos(ctx, camera.x + camera.w / 2.0 - BUTTON_WIDTH / 2.0,
            camera.y + camera.h / 1.75 - BUTTON_HEIGHT / 2.0);

        let giveup_button = self.buttons.get_mut(&ButtonType::GiveUp).unwrap();
        giveup_button.set_pos(ctx, camera.x + camera.w / 2.0 - BUTTON_WIDTH / 2.0,
            camera.y + camera.h / 1.3 - BUTTON_HEIGHT / 2.0);
    }
}
