pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use std::fs::File;
use std::error::Error;

pub type DayFn = fn(&mut File) -> Result<(i32), Box<dyn Error>>;
pub const day_fns: &[DayFn] = &[day1::day1, day2::day2, day3::day3, day4::day4, day5::day5];