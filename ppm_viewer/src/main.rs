extern crate minifb;
extern crate tinyppm;

use minifb::{Key, Window, WindowOptions};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_name = &shared::get_image_name(&args);

    let (width, height, buffer) = tinyppm::ppm_loader::read_image_data(image_name);

    let mut window = Window::new("ppm viewer in Rust",
                                 width,
                                 height,
                                 WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e)});

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}

