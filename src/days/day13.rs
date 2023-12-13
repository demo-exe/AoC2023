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

pub fn part2() -> usize {
    let blocks = utils::read_lines2("input/day13.txt");

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
        let possible_verticals = find_vertical2(&block);
        let possible_verticals = possible_verticals
            .iter()
            .map(|x| (x.0, x.1, false))
            .collect::<Vec<(usize, usize, bool)>>();

        let possible_horizontals = find_horizotal2(&block);
        let possible_horizontals = possible_horizontals
            .iter()
            .map(|x| (x.0, x.1, true))
            .collect::<Vec<(usize, usize, bool)>>();

        let combined = possible_verticals
            .iter()
            .chain(possible_horizontals.iter())
            .collect::<Vec<&(usize, usize, bool)>>();

        // filter those with .1 == 1
        let res1 = combined
            .iter()
            .filter(|x| x.1 == 1)
            .collect::<Vec<&&(usize, usize, bool)>>();

        let winner;

        if res1.len() == 0 {
            assert!(combined.len() == 1);
            winner = combined[0];
        } else {
            assert!(res1.len() == 1);
            winner = res1[0];
        }

        if winner.2 {
            sum += 100 * winner.0;
        } else {
            sum += winner.0;
        }
    }
    //
    // sum
    sum
}

fn compare(line1: &str, line2: &str) -> usize {
    // println!("{} {}", line1, line2);
    let mut diff = 0;

    for (chr1, chr2) in line1.chars().zip(line2.chars()) {
        if chr1 != chr2 {
            diff += 1;
        }
    }

    diff
}

fn verify_vertical2(lines: &Vec<String>, seam: usize) -> usize {
    let mut total_diff = 0;

    for (_i, line) in lines.iter().enumerate() {
        for (j, chr) in line.chars().enumerate().take(seam) {
            let mirrored_j = 2 * seam - j - 1;

            if mirrored_j >= line.len() {
                continue;
            }

            if line.chars().nth(mirrored_j).unwrap() != chr {
                total_diff += 1;
            }
        }
    }

    total_diff
}

fn find_vertical2(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut results = Vec::with_capacity(2);
    for (i1, i2) in (0..lines[0].len()).tuple_windows() {
        let mut seam_diff = 0;
        for line in lines {
            if line.chars().nth(i1).unwrap() != line.chars().nth(i2).unwrap() {
                seam_diff += 1;
            }
        }
        if seam_diff < 2 {
            let diff = verify_vertical2(lines, i2);
            if diff < 2 {
                results.push((i2, diff));
            }
        }
    }

    results
}

fn verify_horizontal2(lines: &Vec<String>, seam: usize) -> usize {
    let mut total_diff = 0;

    for (i, line) in lines.iter().enumerate().take(seam) {
        let mirrored_i = 2 * seam - i - 1;

        if mirrored_i >= lines.len() {
            continue;
        }
        total_diff += compare(&lines[mirrored_i], line);
    }

    total_diff
}

fn find_horizotal2(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut results = Vec::with_capacity(2);

    for ((_, line), (i2, nline)) in lines.iter().enumerate().tuple_windows() {
        if compare(line, nline) < 2 {
            let potential = verify_horizontal2(lines, i2);
            if potential <= 1 {
                results.push((i2, potential));
            }
        }
    }
    results
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
