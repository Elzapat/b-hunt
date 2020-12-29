use ggez::graphics;
use ggez::graphics::Rect;
use ggez::{Context, GameResult};
use rand::Rng;

type Vector2 = cgmath::Vector2<f32>;
type Point2 = cgmath::Point2<f32>;

struct Sprite {
    image: graphics::Image,
    scale: Vector2,
    bounding_box: Rect,
}

pub struct Map {
    width: u32,
    height: u32,
    background: graphics::Mesh,
    nature: Vec<Sprite>,
}

impl Map {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> GameResult<Map> {
        const NB_TREES: u32 = 30;
        const NB_GRASS: u32 = 1000;
        const TREE_RANGE: u32 = 1;
        const GRASS_RANGE: u32 = 4;

        let mut nature = vec![];
        let background = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[Point2::new(0.0, 0.0), Point2::new(0.0, height as f32),
            Point2::new(width as f32, height as f32), Point2::new(width as f32, 0.0)],
            graphics::Color::from_rgb(104, 159, 56)
        )?;

        for i in 0..NB_TREES + NB_GRASS {
            let image: graphics::Image;
            match i {
                0..=NB_TREES => {
                    image = graphics::Image::new(ctx, format!("/tree-{}.png", rand(TREE_RANGE)))?;
                },
                _ => {
                    image = graphics::Image::new(ctx, format!("/grass-{}.png", rand(GRASS_RANGE)))?;
                },
            }
            nature.push(Sprite {
                bounding_box: Rect::new(
                    rand(width),
                    rand(height),
                    image.width() as f32,
                    image.height() as f32
                ),
                image: image,
                scale: Vector2::new(3.0, 3.0),
            }); 
        }

        let map = Map {
            width: width,
            height: height,
            background: background,
            nature: nature
        };

        Ok(map)
    }

    pub fn draw(&self, mut ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(&mut ctx, &self.background, (Point2::new(0.0, 0.0),))?; 

        for thing in self.nature.iter() {
            graphics::draw(ctx, &thing.image, (Point2::new(
                        thing.bounding_box.x as f32, thing.bounding_box.y as f32),))?;
        }

        Ok(())
    }
}

fn rand(max: u32) -> f32 {
    let mut rng = rand::thread_rng();
    let random_f: f64 = rng.gen();
    (random_f * max as f64).round() as f32 
}
