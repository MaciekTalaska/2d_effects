extern crate structopt;
extern crate minifb;
extern crate rand;
extern crate tinyppm;

use minifb::{Key, Window, WindowOptions};
use rand::{thread_rng};
use rand::seq::SliceRandom;
use std::time::Instant;
use structopt::StructOpt;

#[structopt(name = "just a test")]
#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "i", long = "image")]
    image: String,

    #[structopt(short = "t", long = "tilesize", default_value = "8")]
    tile_size: usize,

    #[structopt(short = "f", long = "fx", default_value = "1")]
    fx: u8,
}


pub fn process_framebuffer(src: &[u32], dst: &mut [u32], index: u32, img_width: u32, tile_size: u32, fx: u8) {
    let tiles_per_line = img_width / tile_size;
    let line = index / tiles_per_line;
    let line_offset = line * img_width * tile_size;
    let pixel_offset = (index % tiles_per_line) * tile_size;
    let offset = line_offset + pixel_offset;

    if tile_size == 1 {
        let current = offset as usize;
        dst[current] = src[current];
    } else {
        match fx {
            1 =>
            // TODO:
            //  https://stackoverflow.com/questions/28219231/how-to-idiomatically-copy-a-slice
            //  1) check what is faster: clone_from_slice, copy_from_slice, copy_memory
            //  2) check if there is any performance gain when compared to the double for loop copy
                for y in 0..tile_size {
                    let start = (offset + y * img_width) as usize;
                    let end = start + tile_size as usize;
                    dst[start..end].copy_from_slice(&src[start..end]);
                },
            2 =>
                for y in 0..tile_size {
                    let start = (offset + y * img_width) as usize;
                    let end = start + tile_size as usize;
                    dst[start..end].clone_from_slice(&src[start..end]);
                },
            3 =>
                for y in 0..tile_size {
                    for x in 0..tile_size {
                        let current = (offset + y * img_width + x) as usize;
                        dst[current] = src[current];
                    }
                },

            _ => panic!("only 1 and 2 are supported as fx type!"),

        }

    }
}

fn main() {
    let opt = Opt::from_args();

    let image_name = &opt.image;
    let tile_size = opt.tile_size;

    let (width, height, buffer) = tinyppm::ppm_loader::read_image_data(image_name);

    let mut window = Window::new("ppm viewer in Rust",
                                 width,
                                 height,
                                 WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e)});

    let tile_sqr = tile_size * tile_size;
    let tiles_count = width * height / tile_sqr;

    let mut randomizer : Vec<u32> = (0..tiles_count as u32).collect();
    randomizer.shuffle(&mut thread_rng());
    let mut dest_buffer = vec![0; width * height * 4];

    let mut current_index : usize = 0;

    let time = Instant::now();
    let mut end: bool = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if current_index < tiles_count {
            let v = randomizer[current_index];
            current_index = current_index + 1;
            process_framebuffer(&buffer, &mut dest_buffer, v, width as u32, tile_size as u32, opt.fx);
        } else {
            if !end {
                end = true;
                let t = time.elapsed().as_micros();
                println!("time elapsed: {}", t);
            }
        }
        window.update_with_buffer(&dest_buffer).unwrap();
    }
}
