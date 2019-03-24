extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::{Read, BufRead};
use std::io::BufReader;

fn read_image_data(image_name: &String) -> (usize, usize, Vec<u32>) {
    let file = File::open(image_name).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let (width, height) = read_image_info(&mut reader);

    let mut rgb_buffer: Vec<u8> = Vec::with_capacity(width * height * 3);
    let read_bytes = reader.read_to_end(rgb_buffer.as_mut()).unwrap();

    if read_bytes !=  width * height * 3 {
        println!("error while reading image data");
        std::process::exit(5);
    }

    let buffer = convert_rgb_to_argb(width, height, &mut rgb_buffer);
    (width, height, buffer)
}

fn convert_rgb_to_argb(width: usize, height: usize, rgb_buffer: &mut Vec<u8>) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];
    for index in 0..width * height {
        let pixel = index * 3;
        let r = rgb_buffer[pixel] as u32;
        let g = rgb_buffer[pixel + 1] as u32;
        let b = rgb_buffer[pixel + 2] as u32;
        let color = b + (g << 8) + (r << 16);
        buffer[index] = color;
    }
    buffer
}

fn read_image_info(reader: &mut BufReader<File>) -> (usize, usize) {
    let mut string_buffer = String::new();
    for _i in 0..3 {
        reader.read_line(&mut string_buffer).unwrap();
    }

    let ppm_id = string_buffer.lines().nth(0usize).unwrap();
    validate_ppm_image(ppm_id);

    let image_size = string_buffer.lines().nth(1usize).unwrap().to_string().clone();
    let (width, height) = extract_image_size(image_size);

    let color_depth = string_buffer.lines().nth(2usize).unwrap().to_string().clone();
    validate_color_depth(color_depth);

    (width, height)
}

fn validate_color_depth(bpp_str: String) {
    let bpp = bpp_str.parse::<usize>().expect("image bit depth should be a number");
    if bpp != 255usize {
        println!("only 8bpp RGB images are supported!");
        std::process::exit(4);
    }
}

fn validate_ppm_image(ppm_id: &str) {
    if ppm_id.to_string() != "P6" {
        println!("invalid header");
        std::process::exit(3);
    }
}

fn extract_image_size(size: String) -> (usize, usize) {
    let image_size: Vec<String> = size.split_whitespace().into_iter().map(|w| w.to_string()).collect();
    let width = image_size.first().unwrap()
        .parse::<usize>()
        .expect("image width should be a number");
    let height = image_size.last().unwrap().
        parse::<usize>()
        .expect("image height should be a number");
    (width, height)
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

    let (width, height, buffer) = read_image_data(image_name);

    let mut window = Window::new("ppm viewer in Rust",
                                 width,
                                 height,
                                 WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e)});

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }

}

