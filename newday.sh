#!/bin/bash
day=$(ls src/days | wc -l)
sed -i "$day i pub mod day$day;" src/days/mod.rs
sed -i -r "s/(pub const day_fns: .*)];/\1, day$day::day$day];/g" src/days/mod.rs
touch "input/day$day.in"
sed "s/day1/day$day/g" src/days/day1.rs > src/days/day$day.rs
