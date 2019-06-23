This is a simple effect that displays 2d image in a more fancy way.

Image is being shown by uncovering square areas of the screen. These areas are of predefined size, and the order is random-ish.

It is possible to specify (and change) the size of the tile. Allowed sizes:
- 1
- 2
- 4
- 5
- 8
- 10
- 16
- 20
- 32
- 40

Allowed sizes are dependent on screen (image) size. The sample image is 640 x 480, so square should be small/big enough to make it possible to divide screen into square areas (without any fractions of screen being left).

Running the effect
==================

Several command line options could be used to run this effect with a bit different behavior:

- `i/image <image_name>.ppm` [required] - path to the ppm image to be used for the effect

- `t/tilesize <number>` - specifies tile size. If not given, 8 is used as a default, so that tiles will be squares of size 8 (8 x 8 pixels)

- `f/fx <number` (where number is: 1, 2 or 3) - this is just one of the 3 different ways of copying pixels. This options should be used together with `benchmark` to check which method is the fastest.

- `b/benchmark` this flag indicates if effect should be run in benchmark mode or not. In case the benchmark mode is chosen, effect duration will be calculation and written to the default output (so that it is easy to run it many times via script and collect the results).

*Note:* if you're interested in running benchmarks please have a look at the `benchmark.sh` file.

#### Examples:

`random_squares -i <imagename>.ppm` - runs effect with specified image and default tile size of 8, using default internal copy method (1)

`random_squares -i <imagename>.ppm -t 16` - runs effect with specified image and tile size of 16 pixels by 16 pixels, using default internal copy method (1)

`random_squares -i <imagename>.ppm -t 32 -f 2 -b` - runs effect with specified image and tile size of 32 by 32 pixels, using internal method copy (2) and in benchmark mode.

Dependencies
============

This tiny app depends on:
 - [minifb](https://crates.io/crates/minifb)
 - [tinyppm](https://crates.io/crates/tinyppm)
 - rand
 - structopt