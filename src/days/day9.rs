use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;

struct Program {
    v: Vec<i64>,
    relbase: i64,
    pos: usize
}

impl Program {
    fn r(&mut self, idx: usize) -> i64 {
        if idx >= self.v.len() {
            self.v.resize(idx + 1, 0);
        }
        self.v[idx]
    }
    fn w(&mut self, idx: usize) -> &mut i64 {
        if idx >= self.v.len() {
            self.v.resize(idx + 1, 0);
        }
        &mut self.v[idx]
    }
    fn op(&mut self) -> i64 {
        self.r(self.pos)
    }
    fn read(&mut self, mode: i64, off: usize) -> i64 {
        match mode {
            0 => { let addr = self.r(self.pos + off); self.r(addr as usize) },
            1 => self.r(self.pos + off),
            _ => { let val = self.r(self.pos + off); self.r((val + self.relbase) as usize) } 
        }
    }
    fn write(&mut self, mode: i64, off: usize, val: i64) {
        let addr = match mode {
            0 => self.r(self.pos + off) as usize,
            1 => self.r(self.pos + off) as usize,
            _ => { let val = self.r(self.pos + off); (val + self.relbase) as usize } 
        };
        *self.w(addr) = val;
    }
}

pub fn day9(file: &mut File) -> Result<i32, Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut v: Vec<i64> = Vec::new();
   for l in buf.lines() {
       for s in l.unwrap().split(',') {
           if let Ok(i) = str::parse::<i64>(&s) {
               v.push(i);
           }
       }
   }
   let mut v = Program { v, relbase: 0, pos: 0 };
   let mut input = vec![2];
   let mut output = Vec::new();
    while v.pos < v.v.len() {
        let op = v.op();
        if op == 99 { break; }
        let opcode = op % 100;
        let modes = vec![(op % 1000 - opcode) / 100, (op % 10000 - op % 1000) / 1000, (op % 100000 - op % 10000) / 10000];
        match opcode {
            1 => {
                let (s1, s2) = (v.read(modes[0], 1), v.read(modes[1], 2));
                v.write(modes[2], 3, s1 + s2);
                v.pos += 4;
            }
            2 => {
                let (s1, s2) = (v.read(modes[0], 1), v.read(modes[1], 2));
                v.write(modes[2], 3, s1 * s2);
                v.pos += 4;
            }
            3 => {
                v.write(modes[0], 1, input.pop().unwrap());
                v.pos += 2;
            }
            4 => {
                let out = v.read(modes[0], 1);
                output.push(out);
                v.pos += 2;
            }
            5 => {
                let b = v.read(modes[0], 1) != 0;
                v.pos = if b { v.read(modes[1], 2) as usize } else { v.pos + 3 };
            }
            6 => {
                let b = v.read(modes[0], 1) == 0;
                v.pos = if b { v.read(modes[1], 2) as usize } else { v.pos + 3 };
            }
            7 => {
                let (p1, p2) = (v.read(modes[0], 1), v.read(modes[1], 2));
                v.write(modes[2], 3, if p1 < p2 { 1 } else { 0 });
                v.pos += 4;
            }
            8 => {
                let (p1, p2) = (v.read(modes[0], 1), v.read(modes[1], 2));
                v.write(modes[2], 3, if p1 == p2 { 1 } else { 0 });
                v.pos += 4;
            }
            9 => {
                let inc = v.read(modes[0], 1);
                v.relbase += inc;
                v.pos += 2;
            }
            _ => {}
        };
    }
    println!("{:?}", output);
   Ok(0)
}