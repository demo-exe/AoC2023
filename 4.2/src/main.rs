use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    let mut copies = vec![1; lines.len()];

    lines.iter().enumerate().for_each(|(i, line)| {
        let multiplier = copies[i];

        let winning = &line[10..39];
        let numbers = &line[42..line.len()];

        " 1".trim_start().parse::<i32>().unwrap();
        let mut winning = (0..winning.len())
            .step_by(3)
            .map(|i| winning[i..i + 2].trim_start().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        winning.sort_unstable();

        let numbers = (0..numbers.len())
            .step_by(3)
            .map(|i| numbers[i..i + 2].trim_start().parse::<i32>().unwrap())
            .map(|number| {
                if winning.binary_search(&number).is_ok() {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>();

        for j in i..i + numbers as usize {
            copies[j + 1] += multiplier;
        }
    });

    println!("result = {}", copies.iter().sum::<u32>());
}
