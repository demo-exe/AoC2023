use rayon::prelude::*;

use crate::utils;

pub fn part1() -> isize {
    let lines = utils::read_lines("input/day9.txt");

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
    // dbg!(result);
    // dbg!(&numbers);
    //
    result as isize
}

pub fn part2() -> isize {
    let lines = utils::read_lines("input/day9.txt");

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
        let mut a = layers
            .iter_mut()
            .map(|layer| layer.first().unwrap().to_owned())
            .collect::<Vec<i64>>();
        a.iter_mut().enumerate().for_each(|(i, num)| {
            if i % 2 == 0 {
                *num = 0 - *num
            }
        });

        0 - a.iter().sum::<i64>()
    };

    let result = numbers
        .iter()
        .map(|numbers| extrapolate(numbers))
        .sum::<i64>();

    result as isize
}
