use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::prelude::*;
//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn dbg(s: &Vec<String>) {
    for i in s {
        println!("{}", i);
    }
}

fn main() {
    let mut lines = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(rawlines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in rawlines {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }

    // this expansion step could be done cheaper in terms of memory or time but im lazy
    let mut lines_adjust1 = Vec::with_capacity(lines.len());
    let mut collumn_empty: Vec<bool> = lines[0].chars().map(|x| x != '#').collect();

    for i in lines.iter() {
        let mut empty = true;
        for (j, c) in i.chars().enumerate() {
            if c == '#' {
                empty = false;
                collumn_empty[j] = false;
            }
        }
        if empty {
            lines_adjust1.push(".".repeat(i.len()));
        }
        lines_adjust1.push(i.to_string());
    }

    let mut lines_adjust2 = Vec::with_capacity(lines_adjust1.len());

    let newlen = lines_adjust1[0].len() + collumn_empty.iter().filter(|x| **x).count();

    for line in lines_adjust1.iter() {
        let mut newline = String::with_capacity(newlen);
        for (i, shouldadd) in collumn_empty.iter().enumerate() {
            if *shouldadd {
                newline.push('.');
            }
            newline.push(line.chars().nth(i).unwrap());
        }
        lines_adjust2.push(newline);
    }

    // (x, y)
    let mut stars = Vec::new();

    for (i, line) in lines_adjust2.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push((j as i32, i as i32));
            }
        }
    }

    let mut sum = 0;

    for (i, star1) in stars.iter().take(stars.len() - 1).enumerate() {
        for star2 in stars.iter().skip(i) {
            let len = (star1.0 - star2.0).abs() + (star1.1 - star2.1).abs();
            sum += len;
        }
    }
    dbg!(sum);
}
