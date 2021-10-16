#!/bin/bash
# Scaffold files for an AoC day
# doesn't handle any errors, not even remotely robust
# most of the ops seem idempotent though

# day number to create
day=${1?Usage: new_day <day number>}

# create and switch to new branch
echo "creating branch: feat/day-$day"
git co -b feat/day-$day main

# create empty test input file
echo "creating test input file: input/$day-t.txt"
touch ./input/$day-t.txt

# create module
echo "creating module files: src/day_$day/"
path=./src/day_$day/
mkdir $path
touch $path/mod.rs

# add module to lib
echo "updating lib.rs"
echo "pub mod day_$day;" >> src/lib.rs
echo "formatting"
cargo-fmt

echo "done."
