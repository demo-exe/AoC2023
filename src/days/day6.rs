use crate::utils;

pub fn part1() -> usize {
    let lines = utils::read_lines("input/day6.txt");

    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());
    let distace = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<i32>().unwrap());

    let numbers = time.zip(distace).map(|(t, d)| {
        let t = t as f32;
        let d = d as f32;
        println!("{} {}", t, d);
        // damn, its been a while since i solved quadratic equations XDD
        let x1: f32 = (t + (t * t - 4f32 * d).sqrt()) / 2f32;
        let x2: f32 = (t - (t * t - 4f32 * d).sqrt()) / 2f32;
        println!("{} {}", x1, x2);

        let mut correction = 0;
        if (x1.floor() - x1).abs() < f32::EPSILON {
            correction += -1;
        }

        let result = x1.floor() - x2.floor() + correction as f32;
        println!("x1: {} x2:{}", x1, x2);
        println!("x1 - x2 = {} = {}", result, result.round());

        println!("------------------");

        result
    });

    let mut mult: u64 = 1;

    for number in numbers {
        mult *= number.round() as u64;
    }
    mult as usize
}

pub fn part2() -> usize {
    let lines = utils::read_lines("input/day6.txt");

    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let distance = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let t = time as f64;
    let d = distance as f64;
    println!("{} {}", t, d);
    // damn, its been a while since i solved quadratic equations XDD
    let x1: f64 = (t + (t * t - 4f64 * d).sqrt()) / 2f64;
    let x2: f64 = (t - (t * t - 4f64 * d).sqrt()) / 2f64;
    println!("{} {}", x1, x2);

    let mut correction = 0;
    if (x1.floor() - x1).abs() < f64::EPSILON {
        correction += -1;
    }

    let result = x1.floor() - x2.floor() + correction as f64;
    println!("x1: {} x2:{}", x1, x2);
    println!("x1 - x2 = {} = {}", result, result.round());

    println!("------------------");

    result as usize
}
