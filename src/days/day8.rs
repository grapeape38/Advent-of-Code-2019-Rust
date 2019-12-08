use std::io::{BufRead, BufReader};
use std::fs::File;
use std::error::Error;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
//const WIDTH: usize = 2;
//const HEIGHT: usize = 2;

#[allow(dead_code)]
fn part1(img: &Vec<u8>) -> Result<(i32), Box<dyn Error>> {
    let min_layer = img.chunks(WIDTH * HEIGHT).min_by(
       |l1, l2| 
       l1.iter().filter(|c| **c == 0).count().cmp(&l2.iter().filter(|c| **c == 0).count())
    ).ok_or("No minimum layer found")?;

    println!("{:?}", min_layer);
    let n1s = min_layer.iter().filter(|c: &&u8| **c == 1).count();
    let n2s = min_layer.iter().filter(|c: &&u8| **c == 2).count();
    Ok((n1s * n2s) as i32)
}

#[allow(dead_code)]
fn part2(img: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    let combine_layers = |l1: &[u8], l2: &[u8]| -> Vec<u8> {
        l1.iter().zip(l2.iter()).map(|(p1, p2)|
            if *p1 == 2 { *p2 } else { *p1 }).collect()
    };
    let layer1 = Vec::from(&img[..WIDTH*HEIGHT]);
    let stacked = img[WIDTH*HEIGHT..].chunks(WIDTH * HEIGHT).fold(layer1, 
       |acc, curr| combine_layers(&acc, curr)
    );

    for r in stacked.chunks(WIDTH) {
        let chars: Vec<u8> = r.iter().map(|c| if *c == 1 { '1' as u8 } else { ' ' as u8 }).collect();
        println!("{}", String::from_utf8(chars)?);
    }
    Ok(())
}

pub fn day8(file: &mut File) -> Result<(i32), Box<dyn Error>> {
   let mut input = String::new();
   let mut buf = BufReader::new(file);
   buf.read_line(&mut input)?;
   let img: Vec<u8> = input.chars().filter(|c| *c as u8 >= '0' as u8).map(|c| c as u8 - '0' as u8).collect();
   part2(&img)?;
   Ok(0)
}