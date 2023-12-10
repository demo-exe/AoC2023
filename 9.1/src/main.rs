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

    let numbers = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|i| i.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    let extrapolate = |numbers: &Vec<i64>| {
        // could be improved by only analysing last numbers in each layer
        let mut layers: Vec<Vec<i64>> = Vec::new();
        layers.push(numbers.clone());

        let mut i = 0;
        loop {
            let mut layer: Vec<i64> = Vec::new();
            let mut last = layers[i][0];
            for number in layers[i].iter().skip(1) {
                layer.push(number - last);
                last = *number;
            }
            layers.push(layer);
            i += 1;
            if layers.last().unwrap().iter().all(|&x| x == 0) {
                break;
            }
        }
        layers
            .iter()
            .map(|layer| layer.last().unwrap())
            .sum::<i64>()
    };

    let result = numbers
        .par_iter()
        .map(|numbers| extrapolate(numbers))
        .sum::<i64>();
    dbg!(result);
    // dbg!(&numbers);
}
