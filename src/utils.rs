use std::fs::{self};

pub fn read_lines(filename: &str) -> Vec<String> {
    let lines = fs::read_to_string(filename).unwrap();

    lines.lines().map(|line| line.to_string()).collect()
}
pub fn read_lines2(filename: &str) -> String {
    let lines = fs::read_to_string(filename).unwrap();

    lines
}
