use ggez::{
    Context, GameResult,
    graphics::{ Font, TextFragment, Color, Scale },
    nalgebra::Point2
};

pub struct Text {
    ggez_text: ggez::graphics::Text,
    font: Font,
    scale: Scale,
    pos: Point2<f32>,
    color: Color
}

impl Text {
    pub fn new(ctx: &mut Context, content: String, font: String,
        scale: f32, color: Color) -> GameResult<Text>
    {
        let font = Font::new(ctx, font)?;
        let scale = Scale::uniform(scale);
        let fragment = TextFragment::new(content)
            .font(font)
            .color(color)
            .scale(scale);
        let ggez_text = ggez::graphics::Text::new(fragment);

        let text = Text {
            ggez_text: ggez_text,
            font: font,
            scale: scale,
            color: color,
            pos: Point2::new(0.0, 0.0)
        };

        Ok(text)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        ggez::graphics::draw(ctx, &self.ggez_text, (self.pos,))?;

        Ok(())
    }

    pub fn change_color(&mut self, new_color: Color) {
        self.ggez_text.fragments_mut().get_mut(0).unwrap().color = Some(new_color);
    }

    pub fn change_scale(&mut self, new_scale: f32) {
        self.ggez_text.fragments_mut().get_mut(0).unwrap().scale = Some(ggez::graphics::Scale::uniform(new_scale));
    }

    pub fn set_pos(&mut self, new_pos: Point2<f32>) {
        self.pos = new_pos;
    }

    pub fn width(&self, ctx: &mut Context) -> f32 {
        self.ggez_text.width(ctx) as f32
    }

    pub fn height(&self, ctx: &mut Context) -> f32 {
        self.ggez_text.height(ctx) as f32
    }

    pub fn set_string(&mut self, new_str: String) {
        self.ggez_text = ggez::graphics::Text::new(
            TextFragment::new(new_str)
                .font(self.font)
                .scale(self.scale)
                .color(self.color)
        );
    }

    pub fn contents(&self) -> String {
        self.ggez_text.contents()
    }

    pub fn get_ggez_text(&self) -> &ggez::graphics::Text {
        &self.ggez_text
    }

    pub fn get_pos(&self) -> Point2<f32> {
        self.pos
    }
}
