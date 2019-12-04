use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;
pub fn day1(file: &mut File) -> Result<(i32), Box<dyn Error>> {
   let mut res = 0;
   let buf = BufReader::new(file);
   for l in buf.lines() {
       if let Ok(line) = l {
           if let Ok(mut fuel) = str::parse::<i32>(&line) {
               while fuel > 0 { 
                   let extra = fuel / 3 - 2;
                   if extra > 0 { res += extra; }
                   fuel = extra;
               }
           }
       }
   }
   Ok(res)
}