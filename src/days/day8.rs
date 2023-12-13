use regex::Regex;
use std::collections::HashMap;

use crate::utils;

pub fn part1() -> usize {
    let lines = utils::read_lines("input/day8.txt");

    let directions = lines[0]
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction, = {}", c),
        })
        .cycle();

    let mut map = HashMap::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.iter().skip(2).for_each(|line| {
        let groups = re.captures(line).unwrap();
        let key = groups.get(1).unwrap().as_str().to_string();
        let left = groups.get(2).unwrap().as_str().to_string();
        let right = groups.get(3).unwrap().as_str().to_string();

        map.insert(key, (left, right));
    });

    let mut current = String::from("AAA");
    let mut steps = 0;
    for dir in directions {
        let (left, right) = map.get(&current).unwrap();
        current = if dir == 0 {
            left.to_string()
        } else {
            right.to_string()
        };
        steps += 1;
        if current == "ZZZ" {
            break;
        }
    }

    // println!("Steps: {}", steps);
    steps as usize
}

pub fn part2() -> usize {
    let lines = utils::read_lines("input/day8.txt");

    let directions = lines[0]
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction, = {}", c),
        })
        .cycle();

    let mut map = HashMap::new();
    let mut simul_positions = Vec::new();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();

    lines.iter().skip(2).for_each(|line| {
        let groups = re.captures(line).unwrap();
        let key = groups.get(1).unwrap().as_str().to_string();
        let left = groups.get(2).unwrap().as_str().to_string();
        let right = groups.get(3).unwrap().as_str().to_string();

        if key.ends_with("A") {
            simul_positions.push(key.clone());
        }

        map.insert(key, (left, right));
    });

    let mut steps: Vec<u64> = Vec::new();

    for current in simul_positions.iter_mut() {
        let mut step = 0;
        for dir in directions.clone() {
            let (left, right) = map.get(current).unwrap();
            *current = if dir == 0 {
                left.to_string()
            } else {
                right.to_string()
            };
            step += 1;

            if current.ends_with("Z") {
                steps.push(step);
                break;
            }
        }
    }

    let result = steps.iter().fold(1, |acc, x| num::integer::lcm(acc, *x));
    // dbg!(result);
    //
    // dbg!(steps);
    result as usize
}
