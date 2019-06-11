#!/usr/bin/env bash

# build if executable does not exist

touch results.txt

echo -e "fx,\ttile size,\ttime" >> results.txt
./target/release/random_squares -i ../resources/sky.ppm -t 8 -b -f 1 >> results.txt

./target/release/random_squares -i ../resources/sky.ppm -t 8 -b -f 2 >> results.txt

./target/release/random_squares -i ../resources/sky.ppm -t 8 -b -f 3 >> results.txt

