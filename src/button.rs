use ggez::{
    Context, GameResult,
    graphics::{ Mesh, Rect, Color },
    nalgebra::Point2,
};
use crate::text::Text;

pub struct Button {
    text: Text,
    border: Mesh,
    hitbox: Rect,
    colors: (Color, Color),
    border_thickness: f32,
    hovered: bool
}

impl Button {
    pub fn new(ctx: &mut Context, width: f32, height: f32, x: f32, y: f32, color_when_not_hovered: Color,
        color_when_hovered: Color, thickness: f32, text: String) -> GameResult<Button>
    {
        let hitbox = Rect::new(x, y, width, height);
        let border = Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::stroke(thickness),
            Rect::new(0.0, 0.0, hitbox.w, hitbox.h),
            color_when_not_hovered
        )?;

        let mut inside_text = Text::new(
            ctx,
            text,
            "/Fonts/arial_narrow_7.ttf".to_string(),
            (width + height) / 10.0,
            color_when_not_hovered,
        )?;
        inside_text.set_pos(Point2::new(
            (hitbox.x + (hitbox.w / 2.0)) - (inside_text.width(ctx) / 2.0),
            (hitbox.y + (hitbox.h / 2.0)) - (inside_text.height(ctx) / 2.0)
        )); 

        let button = Button {
            text: inside_text,
            border: border,
            hitbox: hitbox,
            colors: (color_when_not_hovered, color_when_hovered),
            border_thickness: thickness,
            hovered: false
        };

        Ok(button)
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        ggez::graphics::draw(ctx, &self.border, (Point2::new(self.hitbox.x, self.hitbox.y),))?;
        self.text.draw(ctx)?;

        Ok(())
    }

    pub fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        if self.contains(x, y) && !self.hovered {
            self.hovered = true;
            self.text.change_color(self.colors.1);
            self.border = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::stroke(self.border_thickness),
                Rect::new(0.0, 0.0, self.hitbox.w, self.hitbox.h),
                self.colors.1
            ).unwrap();
        } else if !self.contains(x, y) && self.hovered {
            self.hovered = false;
            self.text.change_color(self.colors.0);
            self.border = Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::stroke(self.border_thickness),
                Rect::new(0.0, 0.0, self.hitbox.w, self.hitbox.h),
                self.colors.0
            ).unwrap();
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        self.hitbox.contains(ggez::mint::Point2 { x: x, y: y })
    }

    pub fn width(&self) -> f32 {
        self.hitbox.w
    }

    pub fn height(&self) -> f32 {
        self.hitbox.h
    }

    pub fn set_pos(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.hitbox.x = x;
        self.hitbox.y = y;
        self.text.set_pos(Point2::new(
            (self.hitbox.x + (self.hitbox.w / 2.0)) - (self.text.width(ctx) / 2.0),
            (self.hitbox.y + (self.hitbox.h / 2.0)) - (self.text.height(ctx) / 2.0)
        )); 
    }

    pub fn set_text_scale(&mut self, scale: f32) {
        self.text.change_scale(scale);
    }

    pub fn get_text_pos(&self) -> Point2<f32> {
        self.text.get_pos()
    }

    pub fn get_border(&self) -> &Mesh {
        &self.border
    }

    pub fn get_pos(&self) -> Point2<f32> {
        Point2::new(self.hitbox.x, self.hitbox.y)
    }

    pub fn get_hitbox(&self) -> Rect {
        self.hitbox
    }

    pub fn get_text(&self) -> &Text {
        &self.text
    }
}

