extern crate minifb;
extern crate tinyppm;


use minifb::{Key, Window, WindowOptions};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(".ppm name should be supplied as the argument");
        std::process::exit(1);
    }
    let image_name = args.last().unwrap();
    if ! image_name.ends_with(".ppm") {
        println!("only .ppm images are supported");
        std::process::exit(2);
    }

    let (width, height, buffer) = tinyppm::ppm_loader::read_image_data(image_name);

    let mut window = Window::new("ppm viewer in Rust",
                                 width,
                                 height,
                                 WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e)});

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}

