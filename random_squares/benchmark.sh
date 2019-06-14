#!/usr/bin/env bash

# build if executable does not exist
if ! test -f target/release/random_squares; then
  cargo build --release
fi

# if results.txt already exists - add timestamp to it
# so we save the old, but always newest is: results.txt
if test -f "results.txt"; then
  timestamp=$(date "+%Y.%m.%d-%H.%M%s")
  old="results.txt"
  mv $old $old.$timestamp
fi

touch results.txt

echo -e "fx,\ttile,\ttime" >> results.txt
for tilesize in 2 4 5 8 10 16 20 32 40
do
  for FX in 1 2 3 
  do
    for loops in {1..100}
    do
      ./target/release/random_squares -i ../resources/sky.ppm -t $tilesize -b -f $FX >> results.txt
    done
  done
done
