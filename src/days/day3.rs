use std::io::{self, BufRead, BufReader};
use std::error::Error;
use std::collections::HashSet;
use std::fs::File;
pub fn day3(file: &mut File) -> Result<i32, Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut horiz: HashSet<(i32, i32, i32, i32)> = HashSet::new();
   let mut vert: HashSet<(i32, i32, i32, i32)> = HashSet::new();
   let mut first = true;
   let mut prev_pos;
   let mut cur_pos = (0,0);
   let mut steps = 0;
   let mut min_steps: i32 = 100000000;
   for l in buf.lines() {
       for s in l.unwrap().split(',') {
           if s.len() > 0 {
                prev_pos = cur_pos;
                let (dir, n) = s.split_at(1);
                let amt = str::parse::<i32>(n).unwrap(); 
                steps += amt;
                let d = dir.chars().next().unwrap();
                cur_pos = match d {
                    'R' => (cur_pos.0 + amt, cur_pos.1),
                    'L' => (cur_pos.0 - amt, cur_pos.1),
                    'U' => (cur_pos.0, cur_pos.1 + amt),
                    'D' => (cur_pos.0, cur_pos.1 - amt),
                    _ => cur_pos
                };
                let lr = d == 'R' || d == 'L';
                if first {
                    if lr { horiz.insert((prev_pos.1, prev_pos.0, cur_pos.0, steps)); }
                    else { vert.insert((prev_pos.0, prev_pos.1, cur_pos.1, steps)); }
                }
                else {
                    let lines = if lr { &vert } else { &horiz };
                    for (x, y1, y2, st) in lines.iter() { 
                        let (mut x1, mut x2, y) = if lr { (prev_pos.0, cur_pos.0, cur_pos.1) } else { (prev_pos.1, cur_pos.1, cur_pos.0) };
                        let (x, mut y1, mut y2) = (*x, *y1, *y2);
                        let s1 = steps - i32::abs(x - x2);
                        let s2 = st - i32::abs(y - y2);
                        if y1 > y2 {
                            std::mem::swap(&mut y1, &mut y2);
                        }
                        if x1 > x2 {
                            std::mem::swap(&mut x1, &mut x2);
                        }
                        if x == 0 && y == 0 { continue; }
                        if y >= y1 && y <= y2 && x >= x1 && x <= x2 && (s1 + s2) < min_steps {
                            println!("Intersection: {:?} {:?} {:?} {:?} {:?} {:?}", x1, x2, y1, y2, x, y);
                            println!("num_steps: {:?} {:?}", s1, s2);
                            min_steps = s1 + s2;
                            println!("{:?}", min_steps);
                        }
                    }
                }
           }
       }
       first = false;
       steps = 0;
       cur_pos = (0,0);
   }
   Ok(min_steps)
}