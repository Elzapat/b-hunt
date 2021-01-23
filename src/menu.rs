use ggez::{
    graphics, GameResult, Context,
    graphics::{ Text, TextFragment, Font },
    nalgebra::Point2
};

pub struct Menu {
    title_text: Text,
    press_a_button_text: Text
}

impl Menu {
    pub fn new(ctx: &mut Context) -> GameResult<Menu> {
        let font = Font::new(ctx, "/Fonts/arial_narrow_7.ttf").unwrap();

        let title_text_fragment = TextFragment::new("B-Hunt")
            .font(font)
            .scale(graphics::Scale::uniform(200.0));
        let title_text = Text::new(title_text_fragment);

        let press_a_button_text_fragment = TextFragment::new(
            "Appuyez sur n'importe quelle touche pour commencer...")
            .font(font)
            .scale(graphics::Scale::uniform(30.0));
        let press_a_button_text = Text::new(press_a_button_text_fragment);

        let menu = Menu {
            title_text: title_text,
            press_a_button_text: press_a_button_text
        };

        Ok(menu)
    }

    pub fn update(&mut self) {

    }

    pub fn draw(&self, ctx: &mut Context, screen_size: (f32, f32)) -> GameResult {
        let mut draw_param = graphics::DrawParam::new()
            .dest(Point2::new(
                    screen_size.0 / 2.0 - self.title_text.width(ctx) as f32 / 2.0,
                    screen_size.1 / 2.5 - self.title_text.height(ctx) as f32 / 2.0
            )); 

        graphics::draw(ctx, &self.title_text, draw_param)?;

        draw_param = draw_param.dest(Point2::new(
                screen_size.0 / 2.0 - self.press_a_button_text.width(ctx) as f32 / 2.0,
                screen_size.1 / 1.5 - self.press_a_button_text.height(ctx) as f32 / 2.0
        ));
        graphics::draw(ctx, &self.press_a_button_text, draw_param)?;

        Ok(())
    }
}
