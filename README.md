This repository contains couple of project that are my take on 2d graphics effects (oldschool style - i.e. without any acceleration) in Rust. Each folder contains a separate project.

For the ability to write directly to the framebuffer, I am using [minifb](https://crates.io/crates/minifb)

For the sake of reading images, I use [tinyppm](https://crates.io/crates/tinyppm) - ppm image loader that I have developed for myself, solely for this purpose (its biggest advantage is being super simple).

Important note on dev dependencies:
-----------------------------------

Minifb that is used to obtain direct access to framebuffer has some additional dependencies. In case these are not met, you get 

```
running: "cc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "-Wall" "-Wextra" "-o" "/home/emgieha/projects/_rust/2d_effects/random_squares/target/release/build/minifb-f4459292eec3ce7b/out/src/native/x11/X11MiniFB.o" "-c" "src/native/x11/X11MiniFB.c"
cargo:warning=src/native/x11/X11MiniFB.c:4:10: fatal error: X11/Xcursor/Xcursor.h: No such file or directory
cargo:warning= #include <X11/Xcursor/Xcursor.h>
cargo:warning=          ^~~~~~~~~~~~~~~~~~~~~~~
cargo:warning=compilation terminated.
exit code: 1
```

On Ubuntu the solution is to install small package `libxcursor-dev`.
