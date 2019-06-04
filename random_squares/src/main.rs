#[macro_use]
extern crate structopt;
extern crate minifb;
extern crate rand;
extern crate tinyppm;
extern crate shared;

use minifb::{Key, Window, WindowOptions};
use std::env;
use rand::{thread_rng};
use rand::seq::SliceRandom;
use std::time::Instant;
use structopt::StructOpt;

const DEFAULT_TILE_SIZE: usize = 8;

#[structopt(name = "random squares")]
#[derive(StuctOpt)]
struct Opt {
    #[structopt(short = "t", long = "tilesize", default = 8)]
    tilesize: usize,

    #[structopt(short = "fx", long = "fx", default = 1)]
    fx: u32,
}

pub fn process_framebuffer(src: &[u32], dst: &mut [u32], index: u32, img_width: u32, tile_size: u32) {
    let tiles_per_line = img_width / tile_size;
    let line = index / tiles_per_line;
    let line_offset = line * img_width * tile_size;
    let pixel_offset = (index % tiles_per_line) * tile_size;
    let offset = line_offset + pixel_offset;

    if tile_size == 1 {
        let current = offset as usize;
        dst[current] = src[current];
    } else {
        // TODO:
        //  https://stackoverflow.com/questions/28219231/how-to-idiomatically-copy-a-slice
        //  1) check what is faster: clone_from_slice, copy_from_slice, copy_memory
        //  2) check if there is any performance gain when compared to the double for loop copy
        for y in 0..tile_size {
            let start = (offset + y * img_width) as usize;
            let end = start + tile_size as usize;
            dst[start..end].copy_from_slice(&src[start..end]);
        }

//        for y in 0..tile_size {
//            for x in 0..tile_size {
//                      let current = (offset + y * img_width + x) as usize;
//                      dst[current] = src[current];
//                  }
//        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

//    let opt = Opt::from_args();

//    let image_name = opt.image;
//    let tile_size = opt.tilesize;
    let image_name = &shared::get_image_name(&args);
    let tile_size = shared::get_option_or_default_number::<usize>(&args, 2, DEFAULT_TILE_SIZE);

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
            process_framebuffer(&buffer, &mut dest_buffer, v, width as u32, tile_size as u32);
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
