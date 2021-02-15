use ggez::{
    graphics, Context, GameResult,
    graphics::{ spritebatch, Rect },
    nalgebra::Point2
};

use crate::utils::rand;

struct Sprite {
    image: spritebatch::SpriteBatch,
}

pub struct Tree {
    sprite: graphics::Image,
    hitbox: Rect,
    position: Point2<f32>,
    draw_before_fabien: bool
}

impl Tree { 
    pub fn get_hitbox(&self) -> Rect { self.hitbox } 
    pub fn draw_before_fabien(&mut self, draw: bool) { self.draw_before_fabien = draw; }
}

pub struct Map {
    width: f32,
    height: f32,
    background: graphics::Mesh,
    grass: Vec<Sprite>,
    trees: Vec<Tree>,
}

impl Map {
    pub fn new(ctx: &mut Context, width: f32, height: f32) -> GameResult<Map> {
        const NUMBERS_OF: [u16; 2] = [1000, 30];
        let environment = ["grass", "tree"];
        let ranges: [u16; 2] = [4, 1];

        let background = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[Point2::new(0.0, 0.0), Point2::new(0.0, height as f32),
            Point2::new(width as f32, height as f32), Point2::new(width as f32, 0.0)],
            graphics::Color::from_rgb(104, 159, 56)
        )?;

        let mut grass = vec![];
        let mut trees = vec![];

        // All of this is really ugly code because I had to change a few things
        // multiples times to accomodate for other aspects of the game.
        // But does it work? Uh, I hope it does.
        for (i, env) in environment.iter().enumerate() {
            for id in 0..=ranges[i] {
                let image = graphics::Image::new(ctx, format!("/{}_{}.png", env, id))?;
                let (sprite_width, sprite_height) = (image.width(), image.height());
                let mut spritebatch = spritebatch::SpriteBatch::new(image);

                for _ in 0..NUMBERS_OF[i] {
                    let x = rand(width);
                    let y = rand(height);

                    if *env == "tree" {
                        let hitbox = Rect::new(
                            x,
                            y + (3 * sprite_height) as f32 / 5.0,
                            sprite_width as f32,
                            (2 * sprite_height) as f32 / 5.0
                        );

                        trees.push(Tree {
                            sprite: graphics::Image::new(ctx, format!("/{}_{}.png", env, id))?,
                            hitbox: hitbox,
                            position: Point2::new(x, y),
                            draw_before_fabien: true,
                        });
                    } else {
                        let param = graphics::DrawParam::new()
                            .dest(Point2::new(x, y));
                        spritebatch.add(param);
                    }
                }
                let mut sprite = Sprite {
                    image: spritebatch
                };
                let param = graphics::DrawParam::new();
                sprite.image.add(param);
                match *env {
                    "grass" => grass.push(sprite),
                    _ => {}
                }
            }
        }

        let map = Map {
            width: width,
            height: height,
            background: background,
            grass: grass,
            trees: trees,
        };

        Ok(map)
    }

    pub fn draw(&self, ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(ctx, &self.background, (cgmath::Point2::<f32>::new(0.0, 0.0),))?; 
        for grass in self.grass.iter() {
            let param = graphics::DrawParam::new()
                .dest(Point2::new(0.0, 0.0));
            graphics::draw(ctx, &grass.image, param)?; 
        }

        Ok(())
    }

    pub fn draw_trees_before(&self, ctx: &mut Context) -> ggez::GameResult {
        for tree in self.trees.iter() {
            if tree.draw_before_fabien {
                let param = graphics::DrawParam::new()
                    .dest(Point2::new(tree.position.x, tree.position.y));
                graphics::draw(ctx, &tree.sprite, param)?;
            }
        }

        Ok(())
    }

    pub fn draw_trees_after(&self, ctx: &mut Context) -> ggez::GameResult {
        for tree in self.trees.iter() {
            if !tree.draw_before_fabien {
                let param = graphics::DrawParam::new()
                    .dest(Point2::new(tree.position.x, tree.position.y));
                graphics::draw(ctx, &tree.sprite, param)?;
            }
        }

        Ok(())
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_trees(&mut self) -> &mut Vec<Tree> {
        &mut self.trees
    }
}
