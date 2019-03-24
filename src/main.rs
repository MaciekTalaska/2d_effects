extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::{SeekFrom, Read, Seek};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;


fn main() {
    let args: Vec<String> = env::args().collect();
    let image_name = args.last().unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("ppm viewer in Rust",
    WIDTH,
    HEIGHT,
    WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e)});

    let mut file = File::open(image_name).unwrap();

    file.seek(SeekFrom::Start(15)).unwrap();

    let mut temp:Vec<u8> = Vec::with_capacity(640 * 480 * 3);
    let read_bytes = file.read_to_end(temp.as_mut()).unwrap();
    assert_eq!(read_bytes, WIDTH * HEIGHT * 3);

    for index in 0..WIDTH * HEIGHT {
        let pixel = index * 3;
        let r = temp[pixel] as u32;
        let g = temp[pixel+1] as u32;
        let b = temp[pixel+2] as u32;
        let color = b + (g << 8) + (r <<16);
        buffer[index] = color;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }

}

