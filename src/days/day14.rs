use std::collections::HashMap;

use crate::utils;

pub fn part2() -> usize {
    let blocks = utils::read_lines2("input/day14.txt");
    //
    // let blocks = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";

    let blocks = blocks
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let mut blocks = blocks[0].clone();

    // find cycle
    let mut cycle = 0;
    let mut pre = 0;
    let mut map: HashMap<Vec<String>, usize> = HashMap::new();
    for i in 0..10000 {
        // absolutely barbaric XD
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);

        let entry = map.get(&blocks);
        if entry.is_some() {
            cycle = i - entry.unwrap();
            pre = *entry.unwrap();
            break;
        }
        map.insert(blocks.clone(), i);
    }
    let asd = (1000000000 - pre) % cycle;

    for _ in 0..(asd - 1) {
        // absolutely barbaric XD
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = tilt_west(blocks);
        blocks = rotate_right(blocks);
        blocks = rotate_right(blocks);
    }
    calc_load(&blocks)
}

fn calc_load(input: &Vec<String>) -> usize {
    let mut sum = 0;
    let len = input.len();
    for (i, row) in input.iter().enumerate() {
        let weight = len - i;
        for c in row.chars() {
            if c == 'O' {
                sum += weight;
            }
        }
    }
    sum
}

fn tilt_west(input: Vec<String>) -> Vec<String> {
    let mut output = Vec::with_capacity(input.len());

    for row in input {
        let mut new_row = String::with_capacity(row.len());

        let mut starting = 0;
        let mut rocks = 0;

        for (i, c) in row.chars().enumerate() {
            match c {
                '#' => {
                    for _ in 0..rocks {
                        new_row.push('O');
                    }
                    for _ in 0..(i - starting - rocks) {
                        new_row.push('.');
                    }
                    new_row.push('#');
                    starting = i + 1;
                    rocks = 0;
                }
                'O' => {
                    rocks += 1;
                }
                _ => {}
            }
            if i == row.len() - 1 && c != '#' {
                for _ in 0..rocks {
                    new_row.push('O');
                }
                for _ in 0..(i - starting + 1 - rocks) {
                    new_row.push('.');
                }
            }
        }

        output.push(new_row);
    }

    output
}

fn rotate_right(input: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for i in 0..input[0].len() {
        let mut row = String::new();
        for j in (0..input.len()).rev() {
            row.push(input[j].chars().nth(i).unwrap());
        }
        output.push(row);
    }
    output
}

pub fn part1() -> usize {
    let blocks = utils::read_lines2("input/day14.txt");
    //
    //     let blocks = "O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....";

    let blocks = blocks
        .split("\n\n")
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    tilt_north_load(&blocks[0])
}

fn load(platform_len: usize, start_row: usize, rocks_num: usize) -> usize {
    let mut sum = 0;
    let start = platform_len - start_row - rocks_num + 1;
    let end = platform_len - start_row + 1;
    for i in start..end {
        sum += i;
    }
    sum
}

fn tilt_north_load(block: &Vec<String>) -> usize {
    let mut total = 0;
    for collumn in 0..block[0].len() {
        let mut sum = 0;
        let mut stopping_point = 0;
        let mut rocks_num = 0;
        for row in 0..block.len() {
            match block[row].chars().nth(collumn).unwrap() {
                '#' => {
                    sum += load(block.len(), stopping_point, rocks_num);
                    stopping_point = row + 1;
                    rocks_num = 0;
                }
                'O' => {
                    rocks_num += 1;
                }
                _ => {}
            }

            if row == block.len() - 1 {
                sum += load(block.len(), stopping_point, rocks_num);
            }
        }
        total += sum;
    }

    total
}
