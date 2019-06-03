This repository contains couple of project that are my take on 2d graphics effects (oldschool style - i.e. without any acceleration) in Rust. Each folder contains a separate project.

For the ability to write directly to the framebuffer, I am using [minifb](https://crates.io/crates/minifb)
For the sake of reading images, I use [tinyppm](https://github.com/MaciekTalaska/tinyppm) - ppm image loader that I have developed for myself, solely for this purpose (is super simple).
