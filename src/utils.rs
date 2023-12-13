use std::fs::{self};

pub fn read_lines(filename: &str) -> Vec<String> {
    let lines = fs::read_to_string(filename).unwrap();

    lines.lines().map(|line| line.to_string()).collect()
}
