
extern crate minifb;
extern crate rand;

use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: usize = 1920/4;
const HEIGHT: usize = 1080/4;

fn main() {
    let mut fb = fb::FB::new( WIDTH, HEIGHT );
//    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("ggj19 - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                           resize: true,
                                           scale: Scale::X4,
                                           ..WindowOptions::default()
                                }
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut game = game::Game::new();
    let mut input = input::Input::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
//        for i in buffer.iter_mut() {
//            *i = 0; // write something more funny here!
//        }

        input.update_keys( &window.get_keys() );
        game.update( &mut input );
        game.render( &mut fb );
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&fb.buffer()).unwrap();
    }	
}

mod bobs;
mod fb;
mod game;
mod input;
