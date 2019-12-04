use std::io::{self, BufRead, BufReader};
use std::error::Error;
use std::fs::File;
pub fn day2(file: &mut File) -> Result<i32, Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut v: Vec<i32> = Vec::new();
   for l in buf.lines() {
       for s in l.unwrap().split(',') {
           if let Ok(i) = str::parse::<i32>(&s) {
               v.push(i);
           }
       }
   }
   //println!("{:?}", v);
   let mut pos = 0;
   let v2 = v.clone();
   for x in 0..100 {
       for y in 0..100 {
           v[1] = x as i32;
           v[2] = y as i32;
            while pos < v.len() {
                let op = v[pos];
                if op == 99 { break; }
                let (s1,s2) = (v[pos + 1] as usize, v[pos + 2] as usize);
                let dst = v[pos + 3] as usize;
                if op == 1 {
                    v[dst] = v[s1] + v[s2];
                }
                else if op == 2 {
                    v[dst] = v[s1] * v[s2];
                }
                pos += 4;
            }
            println!("{:?} {:?} : {:?}", x, y, v[0]);
            v = v2.clone();
            pos = 0;
       }
   }
   Ok(0)
}