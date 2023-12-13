use rayon::prelude::*;

use crate::utils;

pub fn part1() -> usize {
    let lines = utils::read_lines("input/day4.txt");

    let result = lines
        .par_iter()
        .map(|line| {
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

            if numbers > 0 {
                2u32.pow(numbers - 1)
            } else {
                0
            }
        })
        .sum::<u32>();

    result as usize
}

pub fn part2() -> usize {
    let lines = utils::read_lines("input/day4.txt");

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

    copies.iter().sum::<u32>() as usize
}
