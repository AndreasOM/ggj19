#![feature(duration_float)]

extern crate minifb;
extern crate rand;

use std::time::{Duration, Instant};

use minifb::{Key, Scale, WindowOptions, Window};

const WIDTH: isize = 480; //1920/4;
const HEIGHT: isize = 270; //1080/4;

fn main() {
    let mut fb = fb::FB::new( WIDTH, HEIGHT );
//    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("ggj19 - ESC to exit",
                                 WIDTH as usize,
                                 HEIGHT as usize,
                                 WindowOptions {
                                           resize: false,
                                           borderless: false,
                                           title: false,
                                           scale: Scale::X4,
                                           ..WindowOptions::default()
                                }
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut game = game::Game::new();
    let mut input = input::Input::new();

    let mut last_time = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
//        for i in buffer.iter_mut() {
//            *i = 0; // write something more funny here!
//        }

        let time_step = last_time.elapsed().as_float_secs() as f32;
        last_time = Instant::now();
        input.update_keys( &window.get_keys() );
        game.update( time_step, &mut input );
        game.render( &mut fb );
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&fb.buffer()).unwrap();
    }	
}

mod bobs;
mod counter;
mod fb;
mod game;
mod input;
