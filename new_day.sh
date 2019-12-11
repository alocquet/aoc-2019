#!/bin/sh

git checkout master
git pull
git checkout -b "feature/day$1"
mkdir src/advent/day$1
cp template.rs src/advent/day$1/mod.rs
echo "pub mod day$1;" >> src/advent/mod.rs
curl --cookie "session=$2" https://adventofcode.com/2019/day/$1/input > src/advent/day$1/input.txt