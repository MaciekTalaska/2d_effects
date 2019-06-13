#!/usr/bin/env bash

# build if executable does not exist

touch results.txt

echo -e "fx,\ttile,\ttime" >> results.txt
#for tilesize in 2 4 5 8 10 16 20 32 40
for tilesize in 32 40
do
  for FX in 1 2 3 
  do
    for loops in {1..2}
    do
      ./target/release/random_squares -i ../resources/sky.ppm -t $tilesize -b -f $FX >> results.txt
    done
  done
done
