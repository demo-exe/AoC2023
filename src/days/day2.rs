use rayon::prelude::*;

use crate::utils::read_lines;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

#[allow(dead_code)]
const NUM_RED: usize = 12;
#[allow(dead_code)]
const NUM_GREEN: usize = 13;
#[allow(dead_code)]
const NUM_BLUE: usize = 14;

pub fn part2() -> usize {
    let alines = read_lines("input/day2.txt");

    // pariter
    let sum = alines
        .par_iter()
        .enumerate()
        .map(|(_id, line)| {
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

    sum as usize
}
