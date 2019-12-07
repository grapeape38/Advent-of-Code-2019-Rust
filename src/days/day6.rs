use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;
#[allow(dead_code)]
pub fn toposort(node: usize, adjlist: &Vec<Vec<usize>>, visited: &mut Vec<bool>, 
    order: &mut Vec<usize>) 
{
    if !visited[node] {
        visited[node] = true;
        for vj in &adjlist[node] {
            toposort(*vj, adjlist, visited, order);
        }
        order.push(node);
    }
}

#[allow(dead_code)]
pub fn toposort_helper(adjlist: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut visited = vec![false; adjlist.len()];
    let mut order: Vec<usize> = Vec::new();
    for i in 0..adjlist.len() {
        toposort(i, adjlist, &mut visited, &mut order);
    }
    order.reverse();
    order
}

#[allow(dead_code)]
pub fn add_depths(node: usize, adjlist: &Vec<Vec<usize>>, visited: &mut Vec<bool>, depth: usize, res: &mut i32) {
    if !visited[node] {
        visited[node] = true;
        *res += depth as i32;
        for vj in &adjlist[node] {
            add_depths(*vj, adjlist, visited, depth + 1, res);
        }
    }
}

pub fn euler(node: usize, adjlist: &Vec<Vec<usize>>, visited: &mut Vec<bool>, depth: i32,
    order: &mut Vec<usize>, heights: &mut Vec<i32>, first: &mut Vec<usize>) {
    visited[node] = true;
    heights[node] = depth;
    first[node] = order.len();
    order.push(node);
    for vj in &adjlist[node] {
        if !visited[*vj] {
            euler(*vj, adjlist, visited, depth + 1, order, heights, first);
            order.push(node);
        }
    }
}

pub fn day6(file: &mut File) -> Result<(i32), Box<dyn Error>> {
   let buf = BufReader::new(file);
   let mut next_id: usize = 0;
   let mut names: HashMap<String, usize> = HashMap::new();
   let mut adjlist: Vec<Vec<usize>> = Vec::new();
   let mut you_orb = 0;
   let mut san_orb = 0;
   for l in buf.lines() {
       if let Ok(line) = l {
           let mut sp = line.split(')');
           let (mut s1, s2) = (sp.next().unwrap(), sp.next().unwrap());
           s1 = &s1[..s1.len()];
           let id1 = *names.entry(s1.to_string()).or_insert_with(|| {
               adjlist.push(Vec::new());
               next_id += 1; next_id - 1});
           let id2 = *names.entry(s2.to_string()).or_insert_with(|| {
               adjlist.push(Vec::new());
               next_id += 1; next_id - 1});
            if s2 == "YOU" {
                you_orb = id1;
            }
            if s2 == "SAN" {
                san_orb = id1;
            }
            adjlist[id1].push(id2);
       }
   }
   let order = toposort_helper(&adjlist);
   //println!("{:?} {:?}", names, order);
   let mut visited = vec![false; adjlist.len()];
   let mut euler_order = Vec::new();
   let mut heights = vec![0; adjlist.len()]; 
   let mut first = vec![0; adjlist.len()];
   for node in order {
       if !visited[node] {
           euler(node, &adjlist, &mut visited, 0, &mut euler_order, &mut heights, &mut first);
       }
   }
   if first[you_orb] > first[san_orb] {
       std::mem::swap(&mut you_orb, &mut san_orb);
   } 
   //let mut lca = 0;
   let mut min_height = heights[you_orb];
   //println!("{:?} {:?}", you_orb, san_orb);
   //println!("Tour: ");
   for index in first[you_orb]..first[san_orb]+1 {
        //println!("{:?}", euler_order[index]);
       if heights[euler_order[index]] < min_height {
           //lca = euler_order[index];
           min_height = heights[euler_order[index]];
       }
   }
   //println!("{:?}", names);
   //println!("{:?} {:?} {:?} {:?}", lca, heights[you_orb], heights[san_orb], min_height);
   let res = heights[you_orb] + heights[san_orb] - 2 * min_height;
   Ok(res)
}