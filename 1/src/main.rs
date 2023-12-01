use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::prelude::*;

fn main() {
    let mut alines = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                alines.push(ip);
            }
        }
    }

    #[derive(Debug)]
    enum Transition {
        End(u32),
        Continue(),
    }

    let mut fsm = HashMap::<String, Transition>::from([
        ("1".to_string(), Transition::End(1)),
        ("2".to_string(), Transition::End(2)),
        ("3".to_string(), Transition::End(3)),
        ("4".to_string(), Transition::End(4)),
        ("5".to_string(), Transition::End(5)),
        ("6".to_string(), Transition::End(6)),
        ("7".to_string(), Transition::End(7)),
        ("8".to_string(), Transition::End(8)),
        ("9".to_string(), Transition::End(9)),
        ("one".to_string(), Transition::End(1)),
        ("two".to_string(), Transition::End(2)),
        ("three".to_string(), Transition::End(3)),
        ("four".to_string(), Transition::End(4)),
        ("five".to_string(), Transition::End(5)),
        ("six".to_string(), Transition::End(6)),
        ("seven".to_string(), Transition::End(7)),
        ("eight".to_string(), Transition::End(8)),
        ("nine".to_string(), Transition::End(9)),
    ]);

    for digit in vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ] {
        let mut accumulator = String::with_capacity(10);
        for c in digit.chars() {
            accumulator.push(c);
            if !fsm.contains_key(&accumulator) {
                fsm.insert(accumulator.clone(), Transition::Continue());
            }
        }
    }

    let result = alines
        .par_iter()
        .map(|line| -> u32 {
            let mut first: Option<u32> = None;
            let mut last: u32 = 0;
            let mut accumulator: Vec<char> = Vec::with_capacity(10);

            // let line = String::from("eightwo");

            let mut i = 0;
            let finish = line.len() - 1;

            loop {
                let c = line.chars().nth(i).unwrap();

                // if c.is_numeric() {
                //     last = c.to_digit(10).unwrap();
                //     if first.is_none() {
                //         first = Some(last);
                //     }
                //     accumulator.clear();
                //
                // }

                accumulator.push(c);
                // dbg!(&accumulator);
                if let Some(transition) = fsm.get(&accumulator.iter().collect::<String>()) {
                    match transition {
                        Transition::End(digit) => {
                            last = *digit;
                            if first.is_none() {
                                first = Some(last);
                            }
                            let len = accumulator.len();
                            accumulator.clear();
                            i -= len - 1;
                        }
                        Transition::Continue() => {}
                    }
                } else {
                    let len = accumulator.len();
                    accumulator.clear();
                    i -= len - 1;
                    // accumulator.push(c);
                }

                i += 1;
                if i > finish {
                    break;
                }
            }

            // line.chars().for_each(|c| {});
            let sum = first.unwrap() * 10 + last;
            // println!("{} -> {}", line, sum);
            sum
        })
        .sum::<u32>();

    println!("result: {}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
