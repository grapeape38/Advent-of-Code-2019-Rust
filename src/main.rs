extern crate clap;
mod days;
use std::error::Error;
use std::fs::File;
use days::day_fns;
use clap::{App, Arg};

const MAX_DAYS: usize = 5;

fn main() -> Result<(), Box<dyn Error>> {
     let matches = App::new("Advent of Code 2019")
        .arg(Arg::with_name("run").short("r").takes_value(true))
        .get_matches();

    let day_num = matches.value_of("run").ok_or("Supply which day's challenge to run")?;
    let day_num = str::parse::<usize>(day_num)?; 
    if day_num > MAX_DAYS {
        return Err("invalid day".into());
    }
    let mut f = File::open(&format!("input/day{}.in", day_num))
        .map_err(|_| format!("File for day {:?} not found in inputs directory", day_num))?;
    let res = (day_fns[day_num - 1])(&mut f);
    match res {
        Ok(res) => {
            println!("Result: {:?}", res);
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}