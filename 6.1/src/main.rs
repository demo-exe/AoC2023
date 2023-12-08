use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

//
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut lines = Vec::new();
    // File hosts.txt must exist in the current path
    if let Ok(rawlines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in rawlines {
            if let Ok(ip) = line {
                lines.push(ip);
            }
        }
    }

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
    dbg!(mult);
}
