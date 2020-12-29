extern crate sfml;

use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

mod resource_manager; use resource_manager::*;

fn main() {

//    let mut tex_holder = ResourceManager::<&str, Texture>::new();
 //   tex_holder.load("map", "resources/map.png");

    let background = Texture::from_file("resources/map.png").unwrap();
    let mut window = RenderWindow::new(
        (800, 600),
        "Fabi Pew Pew",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_framerate_limit(30);

    let mut i = 0.0;
    while window.is_open() {
        while let Some(e) = window.poll_event() {
            match e {
                Event::Closed => window.close(),
                _ => {},
            }
        }

        window.clear(Color::BLACK);
        window.draw(&Sprite::with_texture(&background));
        window.set_view(&View::new(Vector2f::new(i, 100.0), Vector2f::new(100.0, 100.0)));
        window.display();
        i += 1.0;
    }
}
