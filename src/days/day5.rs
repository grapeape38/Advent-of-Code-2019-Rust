use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
pub fn day5(file: &mut File) -> Result<i32, Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut v: Vec<i32> = Vec::new();
   for l in buf.lines() {
       for s in l.unwrap().split(',') {
           if let Ok(i) = str::parse::<i32>(&s) {
               v.push(i);
           }
       }
   }
   let mut input = vec![5];
   let mut output = Vec::new();
   let mut pos = 0;
    while pos < v.len() {
        let op = v[pos];
        if op == 99 { break; }
        let opcode = op % 100;
        let modes = vec![op % 1000 - opcode > 0, op % 10000 - op % 1000 > 0, op % 100000 - op % 10000 > 0];
        println!("{:?} {:?} {:?} {:?}", pos, op, opcode, modes);
        let get_val = |param: bool, pos: usize| {
            if param { v[pos] } else { v[v[pos] as usize] }
        };
        match opcode {
            1 => {
                let dst =  v[pos + 3] as usize;
                let (s1, s2) = (get_val(modes[0], pos + 1), get_val(modes[1], pos + 2));
                v[dst] = s1 + s2;
                pos += 4;
            }
            2 => {
                let dst = v[pos + 3] as usize;
                let (s1, s2) = (get_val(modes[0], pos + 1), get_val(modes[1], pos + 2));
                v[dst] = s1 * s2;
                pos += 4;
            }
            3 => {
                let dst = v[pos + 1] as usize;
                v[dst] = input.pop().unwrap();
                pos += 2;
            }
            4 => {
                let out = get_val(modes[0], pos + 1);
                output.push(out);
                pos += 2;
            }
            5 => {
                let b = get_val(modes[0], pos + 1) != 0;
                pos = if b { get_val(modes[1], pos + 2) as usize } else { pos + 3 };
            }
            6 => {
                let b = get_val(modes[0], pos + 1) == 0;
                pos = if b { get_val(modes[1], pos + 2) as usize } else { pos + 3 };
            }
            7 => {
                let (p1, p2) = (get_val(modes[0], pos + 1), get_val(modes[1], pos + 2));
                let dst = v[pos + 3] as usize;
                v[dst] = if p1 < p2 { 1 } else { 0 };
                pos += 4;
            }
            8 => {
                let (p1, p2) = (get_val(modes[0], pos + 1), get_val(modes[1], pos + 2));
                let dst = v[pos + 3] as usize;
                v[dst] = if p1 == p2 { 1 } else { 0 };
                pos += 4;
            }
            _ => {}
        };
    }
    println!("{:?}", output);
   Ok(0)
}