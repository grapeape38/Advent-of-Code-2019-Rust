use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

#[derive(PartialEq)]
enum Slope {
    Val(f32, bool),
    Up,
    Down,
}

impl Slope {
    fn new(rise: i32, run: i32) -> Self {
        if run == 0 {
            if rise > 0 { Slope::Up } else { Slope::Down }
        }
        else { Slope::Val(rise as f32 / run as f32, run > 0) }
    }
}

pub fn day10(file: &mut File) -> Result<(i32), Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut coords: Vec<(i32, i32)> = Vec::new();
   let mut r = 0;
   for l in buf.lines() {
       if let Ok(line) = l {
           for (c, ch) in line.chars().enumerate() {
               if ch == '#' { coords.push((c as i32, r as i32)); }
           }
           r += 1;
       }
   }
   //println!("{:?}", coords);
   let num_sight = |a: &&(i32, i32)| {
       let mut s1: HashSet<(i32, i32)> = HashSet::new();
       let mut s2: HashSet<(i32, i32)> = HashSet::new();
       s2.insert(**a);
       for (c, r) in &coords {
           if s2.contains(&(*c, *r)) { continue; }
           s1.insert((*c, *r));
           let d = (c - a.0, r - a.1);
           let slope = Slope::new(d.1, d.0);
           for (c2, r2) in &coords {
                let d2 = (c2 - a.0, r2 - a.1);
                let slope2 = Slope::new(d2.1, d2.0);
                if slope == slope2 { 
                    //println!("From asteroid {:?}, ({:?} {:?}) and ({:?} {:?}) are colinear", a, c, r, c2, r2);
                    s2.insert((*c2, *r2));
                }
           }
       }
       //println!("{:?}, count = {:?}", a, s1.len());
       s1.len()
   };

   let mina = *coords.iter().max_by_key(num_sight).ok_or("No minimum")?;
   println!("{:?} {:?}", mina, num_sight(&&mina));
   coords.remove(coords.iter().position(|p| *p == mina).unwrap());
   //let mut rot90: Vec<(i32, i32)> = coords.iter().map(|(x, y)| (mina.1 - y, x - mina.0)).collect();
   let get_quad = |p: &(i32, i32)| {
       if p.0 >= 0 {
           if p.1 > 0 { 3 } else { 0 }
       }
       else {
           if p.1 <= 0 { 1 } else { 2 }
       }
   };
   let rot_90 = |p: &(i32, i32)| {
       (mina.1 - p.1, mina.0 - p.0)
   };
   let pts: Vec<(i32, i32)> = coords.iter().map(|p| rot_90(p)).collect();
   let mut dst_order: HashMap<(i32, i32), usize> = HashMap::new();
   let mut s1: HashSet<(i32, i32)> = HashSet::new();
   for p in &pts {
       if s1.contains(&p) { continue; }
       s1.insert(*p);
       let mut dists: Vec<((i32, i32), i32)> = Vec::new();
       let slope = Slope::new(p.1, p.0);
       dists.push((*p, p.0 * p.0 + p.1 * p.1));
       for p2 in &pts {
           let slope2 = Slope::new(p2.1, p2.0);
           if *p == *p2 { continue; }
           if slope == slope2 {
               s1.insert(*p2);
               dists.push((*p2, p2.0 * p2.0 + p2.1 * p2.1));
           }
       }
       dists.sort_by_key(|(_, dst)| *dst);
       for (i, (p1, _)) in dists.iter().enumerate() {
           dst_order.insert(*p1, i);
       }
       //println!("dists of points of same slope as {:?}, {:?}", p, dists);
   }

   let asort = |p1: &(i32, i32), p2: &(i32, i32)| {
       let (p1, p2) = (rot_90(p1), rot_90(p2));
       let d = dst_order[&p1].cmp(&dst_order[&p2]);
       if d == Ordering::Equal {
            let q = get_quad(&p1).cmp(&get_quad(&p2));
            if q == Ordering::Equal {
                f32::atan2(p2.1 as f32, p2.0 as f32).partial_cmp(&f32::atan2(p1.1 as f32, p1.0 as f32)).unwrap()
            }
            else { q }
        } else { d }
   };
   coords.sort_by(asort);
   println!("{:?}", coords[199]);
   Ok(0)
}