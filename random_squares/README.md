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

1. `cargo run sky.ppm` - this will execute with default tile size set to 8 x 8 pixels

2. `cargo run sky.ppm X` - this will run the effect with the X as specified tile size. Please note that if tile size do not divide whole image without any "leftovers" - the effect will crash.