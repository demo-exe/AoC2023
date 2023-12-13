use itertools::Itertools;

use crate::utils;

pub fn part1() -> usize {
    let blocks = utils::read_lines2("input/day13.txt");
    //
    //     let blocks = "#.##..##.
    // ..#.##.#.
    // ##......#
    // ##......#
    // ..#.##.#.
    // ..##..##.
    // #.#.##.#.
    //
    // #...##..#
    // #....#..#
    // ..##..###
    // #####.##.
    // #####.##.
    // ..##..###
    // #....#..#";

    let blocks = blocks
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut sum = 0;

    for block in blocks {
        if let Some(seam) = find_vertical(&block) {
            sum += seam;
            continue;
        }
        if let Some(seam) = find_horizotal(&block) {
            sum += 100 * seam
        }
    }

    sum
}

fn verify_horizontal(lines: &Vec<String>, seam: usize) -> bool {
    let mut result = true;

    for (i, line) in lines.iter().enumerate().take(seam) {
        let mirrored_i = 2 * seam - i - 1;

        if mirrored_i >= lines.len() {
            continue;
        }

        if &lines[mirrored_i] != line {
            result = false;
            break;
        }
    }

    result
}

fn find_horizotal(lines: &Vec<String>) -> Option<usize> {
    let mut result = None;

    for ((_, line), (i2, nline)) in lines.iter().enumerate().tuple_windows() {
        if line == nline {
            if verify_horizontal(lines, i2) {
                result = Some(i2);
                break;
            }
        }
    }

    result
}

fn verify_vertical(lines: &Vec<String>, seam: usize) -> bool {
    let mut result = true;

    'outer: for (_i, line) in lines.iter().enumerate() {
        for (j, chr) in line.chars().enumerate().take(seam) {
            let mirrored_j = 2 * seam - j - 1;

            if mirrored_j >= line.len() {
                continue;
            }

            if line.chars().nth(mirrored_j).unwrap() != chr {
                result = false;
                break 'outer;
            }
        }
    }

    result
}

fn find_vertical(lines: &Vec<String>) -> Option<usize> {
    let mut result = None;
    for (i1, i2) in (0..lines[0].len()).tuple_windows() {
        let mut all = true;
        for line in lines {
            if line.chars().nth(i1).unwrap() != line.chars().nth(i2).unwrap() {
                all = false;
                break;
            }
        }
        if all {
            if verify_vertical(lines, i2) {
                result = Some(i2);
                break;
            }
        }
    }

    result
}
