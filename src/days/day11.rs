use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;

struct Program {
    v: Vec<i64>,
    relbase: i64,
    pos: usize,
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
    fn execute(&mut self, mut input: Option<i64>) -> (Vec<i64>, bool) {
        let mut output = Vec::new();
        while self.pos < self.v.len() {
            let op = self.op();
            if op == 99 { break; }
            let opcode = op % 100;
            let modes = vec![(op % 1000 - opcode) / 100, (op % 10000 - op % 1000) / 1000, (op % 100000 - op % 10000) / 10000];
            match opcode {
                1 => {
                    let (s1, s2) = (self.read(modes[0], 1), self.read(modes[1], 2));
                    self.write(modes[2], 3, s1 + s2);
                    self.pos += 4;
                }
                2 => {
                    let (s1, s2) = (self.read(modes[0], 1), self.read(modes[1], 2));
                    self.write(modes[2], 3, s1 * s2);
                    self.pos += 4;
                }
                3 => {
                    match input {
                        Some(inp) => { self.write(modes[0], 1, inp);
                                        input = None;
                                        self.pos += 2; }
                        None => { return (output, false); }
                    }
                }
                4 => {
                    let out = self.read(modes[0], 1);
                    output.push(out);
                    self.pos += 2;
                }
                5 => {
                    let b = self.read(modes[0], 1) != 0;
                    self.pos = if b { self.read(modes[1], 2) as usize } else { self.pos + 3 };
                }
                6 => {
                    let b = self.read(modes[0], 1) == 0;
                    self.pos = if b { self.read(modes[1], 2) as usize } else { self.pos + 3 };
                }
                7 => {
                    let (p1, p2) = (self.read(modes[0], 1), self.read(modes[1], 2));
                    self.write(modes[2], 3, if p1 < p2 { 1 } else { 0 });
                    self.pos += 4;
                }
                8 => {
                    let (p1, p2) = (self.read(modes[0], 1), self.read(modes[1], 2));
                    self.write(modes[2], 3, if p1 == p2 { 1 } else { 0 });
                    self.pos += 4;
                }
                9 => {
                    let inc = self.read(modes[0], 1);
                    self.relbase += inc;
                    self.pos += 2;
                }
                _ => {}
            };
        }
        (output, true)
    }
}

pub fn day11(file: &mut File) -> Result<i32, Box<dyn Error>> {
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
   let mut ship: HashMap<(i32, i32), bool> = HashMap::new();
   #[derive(Debug)]
   enum Dir {
       Up, Right, Left, Down 
   };
   #[derive(Debug)]
   struct Robot {
       dir: Dir,
       pos: (i32, i32)
   }
   impl Robot {
       fn go_cw(&mut self) {
            self.dir = match self.dir {
                Dir::Up => Dir::Right, Dir::Right => Dir::Down, Dir::Down => Dir::Left, Dir::Left => Dir::Up
            }
       }
       fn go_ccw(&mut self) {
            self.dir = match self.dir {
            Dir::Up => Dir::Left, Dir::Left => Dir::Down, Dir::Down => Dir::Right, Dir::Right => Dir::Up
        }
       }
       fn advance(&mut self) {
           self.pos = match self.dir {
               Dir::Up => (self.pos.0, self.pos.1 - 1),
               Dir::Left => (self.pos.0 - 1, self.pos.1),
               Dir::Down => (self.pos.0, self.pos.1 + 1),
               Dir::Right => (self.pos.0 + 1, self.pos.1)
           }
       }
   }
   let mut robot = Robot { dir: Dir::Up, pos: (0, 0) };
   ship.insert(robot.pos, true);
   let mut min_white = (0 as i32,0 as i32);
   let mut max_white = (0 as i32,0 as i32);
   loop {
       let (output, done) = v.execute(Some(ship.get(&robot.pos).map(|b| (if *b { 1 } else { 0 } as i64)).unwrap_or(0)));
       if done { break; }
       //println!("{:?} {:?}", robot, output);
       ship.insert(robot.pos, output[0] == 1);
       if output[0] == 1 {
           min_white = (min_white.0.min(robot.pos.0), min_white.1.min(robot.pos.1));
           max_white = (max_white.0.max(robot.pos.0), max_white.1.max(robot.pos.1));
       }
       if output[1] == 0 {
           robot.go_ccw();
       }
       else {
           robot.go_cw();
       }
       robot.advance();
   }
    //println!("{:?}", ship.len());
    println!("{:?} {:?}", min_white, max_white);
   for y in min_white.1..max_white.1 + 1 {
       for x in min_white.0..max_white.0 + 1 {
           print!("{}", ship.get(&(x,y)).map(|b| if *b { '*' } else { ' ' }).unwrap_or(' '));
       }
       println!();
   }
   Ok(0)
}