use ggez::graphics;
use ggez::Context;
use rand::Rng;

type Vector2 = cgmath::Vector2<f32>;
type Point2 = cgmath::Point2<f32>;

struct Sprite {
    image: graphics::Image,
    scale: Vector2,
    x: i32,
    y: i32,
}

pub struct Map {
    width: u32,
    height: u32,
    background: graphics::Mesh,
    nature: Vec<Sprite>,
}

impl Map {
    pub fn new(ctx: &mut Context, width: u32, height: u32) -> Self {
        const NB_TREES: u32 = 10;
        const NB_GRASS: u32 = 100;
        const TREE_RANGE: u32 = 0;
        const GRASS_RANGE: u32 = 4;

        let mut nature = vec![];
        let background = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[Point2::new(0.0, 0.0), Point2::new(0.0, height as f32),
            Point2::new(width as f32, height as f32), Point2::new(width as f32, 0.0)],
            graphics::BLACK,
        ).unwrap();

        for _ in 0..NB_TREES {
            nature.push(Sprite {
                image: graphics::Image::new(ctx, format!("/tree-{}.png", rand(TREE_RANGE))).unwrap(),
                scale: Vector2::new(3.0, 3.0),
                x: rand(width),
                y: rand(height),
            }); 
        }

        for _ in 0..NB_GRASS {
            nature.push(Sprite {
                image: graphics::Image::new(ctx, format!("/grass-{}.png", rand(GRASS_RANGE))).unwrap(),
                scale: Vector2::new(1.0, 1.0),
                x: rand(width),
                y: rand(height),
            })
        }
    
        Map {
            width: width,
            height: height,
            background: background,
            nature: nature
        }
    }

    pub fn draw(&self, mut ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(&mut ctx, &self.background, (Point2::new(0.0, 0.0),))?; 

        for thing in self.nature.iter() {
            graphics::draw(ctx, &thing.image, (Point2::new(thing.x as f32, thing.y as f32),))?;
        }

        Ok(())
    }
}

fn rand(max: u32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_f: f64 = rng.gen();
    (random_f * max as f64).floor() as i32
}
