use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
pub fn run_program(code: &[u32; 5], v: Vec<i64>) -> u32 {
    let mut index = 0;
    let mut res = 0;
    let mut first = vec![true; code.len()];
    let mut vs: Vec<(Vec<i64>, usize)> = vec![(v, 0); code.len()];
    let mut inputs: Vec<Option<i64>> = vec![None; code.len()];
    inputs[0] = Some(0);
    while let Some(input) = inputs[index] {
        let (ref mut v, ref mut p) = vs[index];
        let mut pos = *p;
        while pos < v.len() {
            let op = v[pos];
            if op == 99 { break; }
            let opcode = op % 100;
            let modes = vec![op % 1000 - opcode > 0, op % 10000 - op % 1000 > 0, op % 100000 - op % 10000 > 0];
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
                    if first[index] || inputs[index].is_some() {
                        let dst = v[pos + 1] as usize;
                        v[dst] = 
                            if first[index] { 
                                first[index] = false;
                                code[index] as i64 
                            } else { 
                                inputs[index] = None; 
                                input };
                        pos += 2;
                     }
                     else { break; }
                }
                4 => {
                    let out = get_val(modes[0], pos + 1);
                    inputs[(index + 1) % code.len()] = Some(out);
                    if index == code.len() - 1 {
                        res = out;
                    }
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
        *p = pos;
        index = (index + 1) % code.len();
    }
    println!("code: {:?}, signal sent: {:?}", code, res);
    res as u32
}

pub fn recurse(code: &mut [u32; 5], remaining: &mut Vec<u32>, v: &Vec<i64>, max_signal: &mut u32, max_code: &mut [u32; 5]) {
    if remaining.is_empty() {
        let sig = run_program(code, v.clone());
        if sig > *max_signal {
            *max_signal = sig;
            *max_code = code.clone();
        }
        return;
    }
    let idx = code.len() - remaining.len();
    for i in 0..remaining.len() {
        let next = remaining.remove(i);
        code[idx] = next;
        recurse(code, remaining, v, max_signal, max_code);
        remaining.insert(i, next);
    }
}

pub fn day7(file: &mut File) -> Result<i32, Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut v: Vec<i64> = Vec::new();
   for l in buf.lines() {
       for s in l.unwrap().split(',') {
           if let Ok(i) = str::parse::<i64>(&s) {
               v.push(i);
           }
       }
   }
   let mut max_code = [0,0,0,0,0];
   let mut max_signal = 0;
   recurse(&mut [0,0,0,0,0], &mut vec![5, 6, 7, 8, 9], &v, &mut max_signal, &mut max_code); 
   println!("{:?} {:?}", max_code, max_signal);
   Ok(0)
}