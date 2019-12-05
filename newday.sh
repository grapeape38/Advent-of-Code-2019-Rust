#!/bin/bash
day=$(awk '/const MAX_DAYS/{print $5; }' src/main.rs | rev | cut -c 2 | rev)
day=$(($day+1))
sed -i -r "s/(const MAX_DAYS: usize = ).*;/\1$day;/g" src/main.rs 
sed -i "$day i pub mod day$day;" src/days/mod.rs
sed -i -r "s/(pub const day_fns: .*)];/\1, day$day::day$day];/g" src/days/mod.rs
touch "input/day$day.in"
sed "s/day1/day$day/g" src/days/day1.rs > src/days/day$day.rs
