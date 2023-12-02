use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::prelude::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

const NUM_RED: u32 = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32 = 14;

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

    // pariter
    let sum = alines
        .par_iter()
        .enumerate()
        .map(|(id, line)| {
            let index = &line[5..].find(" ").unwrap();
            let games = &line[6 + index..];
            let games = games.split("; ");

            let (mut red, mut green, mut blue) = (0, 0, 0);

            for game in games {
                let balls = game.split(", ");
                for ball in balls {
                    let num = ball.split_once(" ").unwrap().0;
                    let num = num.parse::<u32>().unwrap();

                    // // if last char is e
                    // if &ball[ball.len() - 1..] == "d" {
                    //     // red
                    //     if num > NUM_RED {
                    //         return 0;
                    //     }
                    // } else if &ball[ball.len() - 1..] == "n" {
                    //     // green
                    //     if num > NUM_GREEN {
                    //         return 0;
                    //     }
                    // } else if &ball[ball.len() - 1..] == "e" {
                    //     // blue
                    //     if num > NUM_BLUE {
                    //         return 0;
                    //     }
                    // }

                    // if last char is e
                    if &ball[ball.len() - 1..] == "d" {
                        // red
                        if num > red {
                            red = num;
                        }
                    } else if &ball[ball.len() - 1..] == "n" {
                        // green
                        if num > green {
                            green = num;
                        }
                    } else if &ball[ball.len() - 1..] == "e" {
                        // blue
                        if num > blue {
                            blue = num;
                        }
                    }
                }
            }
            // dbg!(line);
            // dbg!(red, green, blue);
            // panic!();

            red * green * blue
        })
        .sum::<u32>();

    println!("sum: {}", sum);
}
