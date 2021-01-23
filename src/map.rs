use ggez::{
    graphics, Context, GameResult,
    graphics::{ spritebatch, Rect },
    nalgebra::Point2
};

use crate::utils::rand;

struct Sprite {
    image: spritebatch::SpriteBatch,
    width: u16,
    height: u16
}

pub struct Map {
    width: f32,
    height: f32,
    background: graphics::Mesh,
    nature: Vec<Sprite>,
    hitboxes: Vec<Rect>
}

impl Map {
    pub fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<Map> {
        const NUMBERS_OF: [u16; 2] = [1000, 30];
        let environment = ["grass", "tree"];
        let ranges: [u16; 2] = [4, 1];
        let mut hitboxes = vec![];

        let background = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[Point2::new(0.0, 0.0), Point2::new(0.0, height as f32),
            Point2::new(width as f32, height as f32), Point2::new(width as f32, 0.0)],
            graphics::Color::from_rgb(104, 159, 56)
        )?;

        let mut nature = vec![];
        for (i, env) in environment.iter().enumerate() {
            for id in 0..=ranges[i] {
                let image = graphics::Image::new(ctx, format!("/{}_{}.png", env, id))?;
                let (sprite_width, sprite_height) = (image.width(), image.height());
                let mut spritebatch = spritebatch::SpriteBatch::new(image);
                for _ in 0..NUMBERS_OF[i] {
                    let x = rand(width);
                    let y = rand(height);

                    hitboxes.push(Rect::new(x, y, sprite_width as f32, sprite_height as f32));

                    let param = graphics::DrawParam::new()
                        .dest(Point2::new(x, y));
                    spritebatch.add(param);
                }
                let mut sprite = Sprite {
                    width: sprite_width,
                    height: sprite_height,
                    image: spritebatch
                };
                let param = graphics::DrawParam::new();
                sprite.image.add(param);
                nature.push(sprite);
            }
        }

        let map = Map {
            width: width,
            height: height,
            background: background,
            nature: nature,
            hitboxes: hitboxes
        };

        Ok(map)
    }

    pub fn draw(&self, mut ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(&mut ctx, &self.background, (cgmath::Point2::<f32>::new(0.0, 0.0),))?; 

        for thing in self.nature.iter() {
            let param = graphics::DrawParam::new()
                .dest(Point2::new(0.0, 0.0));
            graphics::draw(ctx, &thing.image, param)?; 
        }

        Ok(())
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_hitboxes(&self) -> &Vec<Rect> {
        &self.hitboxes
    }
}
