use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::error::Error;

fn gen_increasing_p1(digit: usize, n_digit: usize, prev_same: (bool, bool), double: bool, min_max_digits: (&[usize], &[usize]), count: &mut usize, debug: &mut String) {
    if n_digit == min_max_digits.1.len() {
        if double { 
            *count += 1; 
            println!("{}", debug);
        }
        return;
    }
    let min_dig = if prev_same.0 { digit.max(min_max_digits.0[n_digit]) } else { digit };
    let max_dig = if prev_same.1 { min_max_digits.1[n_digit] } else { 9 };
    for i in min_dig..max_dig + 1 {
        debug.push(('0' as u8 + i as u8) as char);
        gen_increasing_p1(i, n_digit + 1, 
            (prev_same.0 && i == min_max_digits.0[n_digit], prev_same.1 && i == min_max_digits.1[n_digit]), n_digit > 0 && 
            (double || i == digit), min_max_digits, count, debug);
        debug.pop();
    }
}

fn gen_increasing_p2(digit: usize, n_digit: usize, prev_same: (bool, bool), repeat_count: usize, 
    mut double: bool, min_max_digits: (&[usize], &[usize]), count: &mut usize, debug: &mut String) 
{
    if n_digit == min_max_digits.1.len() {
        if double || repeat_count == 2{ 
            *count += 1; 
            println!("{}", debug);
        }
        else {
            println!("No match {} {}", debug, repeat_count);
        }
        return;
    }
    let min_dig = if prev_same.0 { digit.max(min_max_digits.0[n_digit]) } else { digit };
    let max_dig = if prev_same.1 { min_max_digits.1[n_digit] } else { 9 };
    for i in min_dig..max_dig + 1 {
        let mut rep = repeat_count;
        debug.push(('0' as u8 + i as u8) as char);
        if n_digit == 0 || i == digit {
            rep += 1;
        }
        else { 
            if repeat_count == 2 {
                double = true;
            }
            rep = 1;
        }
        gen_increasing_p2(i, n_digit + 1, 
            (prev_same.0 && i == min_max_digits.0[n_digit], prev_same.1 && i == min_max_digits.1[n_digit]), rep, double, min_max_digits, count, debug);
        debug.pop();
    }
}



pub fn day4(file: &mut File) -> Result<(i32), Box<dyn Error>> {
    let mut buf = BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line)?;
    let nums: Vec<&str> = line.split('-').collect();
    let (min, max) = (nums[0], nums[1]);
    let max: Vec<usize> = max.chars().map(|c| c as usize- '0' as usize).collect();
    let mut min: Vec<usize> = min.chars().map(|c| c as usize- '0' as usize).collect();
    while min.len() < max.len() {
        min.insert(0, 0);
    }
    println!("{:?} {:?}", min, max);
    let mut count = 0;
    let mut debug = String::new();
    gen_increasing_p2(min[0], 0, (true, true), 0, false, (&min, &max), &mut count, &mut debug);
    Ok(count as i32)
}
