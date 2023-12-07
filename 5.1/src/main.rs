use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
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

#[derive(Debug)]
struct Mapping {
    /* ( input_start, input_end, len ) */
    list: Vec<(isize, isize, isize)>,
}
impl Mapping {
    fn map(self: &Self, number: isize) -> isize {
        for m in &self.list {
            // input_start <= number <= input_start + len
            if m.1 <= number && number <= m.1 + m.2 {
                return (m.0 - m.1) + number;
            }
        }
        return number;
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

    let seeds = lines[0].splitn(2, " ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();

    let mut mappings: Vec<Mapping> = Vec::new();

    for line in lines.iter().skip(2) {
        if line.is_empty() {
            continue;
        }

        if line.ends_with(":") {
            mappings.push(Mapping { list: Vec::new() });
            continue;
        }

        let numbers: (isize, isize, isize) = line
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        mappings.last_mut().unwrap().list.push(numbers);
    }

    let mut results = Vec::new();

    for seed in seeds {
        let mut current = seed;
        for mapping in &mappings {
            // dbg!(current);
            current = mapping.map(current);
        }
        results.push(current);
    }
    // dbg!(results);

    let result = results.iter().min().unwrap();
    dbg!(result);
}
