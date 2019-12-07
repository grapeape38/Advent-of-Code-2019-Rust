extern crate clap;
mod days;
use std::error::Error;
use std::fs::File;
use days::day_fns;
use clap::{App, Arg};

fn run(path: &str, day_num: usize) -> Result<(), Box<dyn Error>> {
    if day_num > day_fns.len() {
        return Err("Invalid day".into());
    }
    let mut f = File::open(path)?;
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

fn main() -> Result<(), Box<dyn Error>> {
     let matches = App::new("Advent of Code 2019")
        .arg(Arg::with_name("run").short("r").takes_value(true))
        .arg(Arg::with_name("test").short("t").takes_value(true))
        .get_matches();
   
    if matches.is_present("run") || matches.is_present("test") {
        if let Some(day_num) = matches.value_of("run") {
            let day_num = str::parse::<usize>(day_num)?; 
            println!("Running day {:?}", day_num);
            run(&format!("input/day{}.in", day_num), day_num)?;
        }
        if let Some(day_num) = matches.value_of("test") {
            let day_num = str::parse::<usize>(day_num)?; 
            println!("Testing day {:?}", day_num);
            run(&format!("test/day{}.in", day_num), day_num)?;
        }
        Ok(())
    }
    else { 
        Err("Pass a day with -r or -t to run/test".into()) 
    }
}
