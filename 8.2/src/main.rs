use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let mut simul_positions = Vec::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.iter().skip(2).for_each(|line| {
        let groups = re.captures(line).unwrap();
        let key = groups.get(1).unwrap().as_str().to_string();
        let left = groups.get(2).unwrap().as_str().to_string();
        let right = groups.get(3).unwrap().as_str().to_string();

        if key.ends_with("A") {
            simul_positions.push(key.clone());
        }

        map.insert(key, (left, right));
    });

    let mut steps: Vec<u64> = Vec::new();

    for current in simul_positions.iter_mut() {
        let mut step = 0;
        for dir in directions.clone() {
            let (left, right) = map.get(current).unwrap();
            *current = if dir == 0 {
                left.to_string()
            } else {
                right.to_string()
            };
            step += 1;

            if current.ends_with("Z") {
                steps.push(step);
                break;
            }
        }
    }

    let result = steps.iter().fold(1, |acc, x| num::integer::lcm(acc, *x));
    dbg!(result);

    dbg!(steps);
}
