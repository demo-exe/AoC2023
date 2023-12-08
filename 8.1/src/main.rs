use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::prelude::*;
use regex::Regex;
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

    let directions = lines[0]
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction, = {}", c),
        })
        .cycle();

    let mut map = HashMap::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.iter().skip(2).for_each(|line| {
        let groups = re.captures(line).unwrap();
        let key = groups.get(1).unwrap().as_str().to_string();
        let left = groups.get(2).unwrap().as_str().to_string();
        let right = groups.get(3).unwrap().as_str().to_string();

        map.insert(key, (left, right));
    });

    let mut current = String::from("AAA");
    let mut steps = 0;
    for dir in directions {
        let (left, right) = map.get(&current).unwrap();
        current = if dir == 0 {
            left.to_string()
        } else {
            right.to_string()
        };
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }

    println!("Steps: {}", steps);
}
