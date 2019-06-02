extern crate minifb;
extern crate rand;
extern crate tinyppm;


use minifb::{Key, Window, WindowOptions};
use std::env;
use rand::{thread_rng};
use rand::seq::SliceRandom;


pub fn process_framebuffer(src: &[u32], dst: &mut [u32], index: u32, img_width: u32, tile_size: u32) {
    let tiles_per_line = img_width / tile_size;
    let line = index / tiles_per_line;
    let line_offset = line * img_width * tile_size;
    let pixel_offset = (index % tiles_per_line) * tile_size;
    let offset = line_offset + pixel_offset;

    // TODO:
    //  https://stackoverflow.com/questions/28219231/how-to-idiomatically-copy-a-slice
    for y in 0..tile_size {
        for x in 0..tile_size {
            let current = (offset + y * img_width + x) as usize;
            let value = src[current];
            dst[current] = value;
        }
    }
}

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

    let tile_size = 8;
    let tile_sqr = tile_size * tile_size;
    let tiles_count = width * height / tile_sqr;

    let mut randomizer : Vec<u32> = (0..tiles_count as u32).collect();
    randomizer.shuffle(&mut thread_rng());
    let mut dest_buffer = vec![0; width * height * 4];

    let mut current_index : usize = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if current_index < tiles_count {
            let v = randomizer[current_index];
            current_index = current_index + 1;
            process_framebuffer(&buffer, &mut dest_buffer, v, width as u32, tile_size as u32);
        }
        window.update_with_buffer(&dest_buffer).unwrap();
    }
}

