use ggez::{
    graphics, GameResult, Context,
    graphics::{ Text, TextFragment, Font },
    nalgebra::Point2
};

pub struct GameOver {
    game_over_text: Text,
    high_score_text: Text,
    current_score_text: Text,
    press_a_button_text: Text
}

impl GameOver {
    pub fn new(ctx: &mut Context, high_score: u32, cur_score: u32) -> GameResult<GameOver> {
        let font = Font::new(ctx, "/Fonts/arial_narrow_7.ttf").unwrap();

        let game_over_fragment = TextFragment::new("Game Over")
            .font(font)
            .scale(graphics::Scale::uniform(200.0));
        let game_over_text = Text::new(game_over_fragment);

        let high_score_fragment = TextFragment::new(format!("Votre meilleur score : {}", high_score))
            .font(font)
            .scale(graphics::Scale::uniform(30.0));
        let high_score_text = Text::new(high_score_fragment);

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
            high_score_text: high_score_text,
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
            screen_size.1 / 3.5 - self.game_over_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::default().dest(dest);
        graphics::draw(ctx, &self.game_over_text, draw_param)?;

        let dest = Point2::new(
            screen_size.0 / 2.0 - self.high_score_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 1.8 - self.high_score_text.height(ctx) as f32 / 2.0
        );
        let draw_param = graphics::DrawParam::default().dest(dest);
        graphics::draw(ctx, &self.high_score_text, draw_param)?;

        let dest = Point2::new(
            screen_size.0 / 2.0 - self.current_score_text.width(ctx) as f32 / 2.0,
            screen_size.1 / 1.6 - self.current_score_text.height(ctx) as f32 / 2.0
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
