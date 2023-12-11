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

const SPACE_MULTIPLIER: i64 = 1000000 - 1;

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
    // UPDATE: damn, they made me do it :/
    let mut collumn_empty: Vec<bool> = lines[0].chars().map(|x| x != '#').collect();
    let mut rows_empty: Vec<bool> = Vec::with_capacity(lines.len());

    for i in lines.iter() {
        let mut empty = true;
        for (j, c) in i.chars().enumerate() {
            if c == '#' {
                empty = false;
                collumn_empty[j] = false;
            }
        }
        if empty {
            rows_empty.push(true);
        } else {
            rows_empty.push(false);
        }
    }

    // (x, y)
    let mut stars = Vec::new();

    let mut row_adjust = 0;
    for (i, line) in lines.iter().enumerate() {
        if rows_empty[i] {
            row_adjust += 1;
        }
        let mut collumn_adjust = 0;
        for (j, c) in line.chars().enumerate() {
            if collumn_empty[j] {
                collumn_adjust += 1;
            }
            if c == '#' {
                stars.push((
                    (j as i64) + collumn_adjust * SPACE_MULTIPLIER,
                    (i as i64) + row_adjust * SPACE_MULTIPLIER,
                ));
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
