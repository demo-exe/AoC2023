use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rayon::prelude::*;
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

#[derive(Debug, PartialEq)]
enum Pipe {
    Start,
    LeftRight,
    TopDown,
    TopRight,
    TopLeft,
    LeftDown,
    RightDown,
    Ground,
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

    let mut map: HashMap<(i32, i32), Pipe> = HashMap::with_capacity(lines.len() * lines[0].len());

    let mut start = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pipe = match c {
                '-' => Pipe::LeftRight,
                '|' => Pipe::TopDown,
                'S' => Pipe::Start,
                'L' => Pipe::TopRight,
                'J' => Pipe::TopLeft,
                '7' => Pipe::LeftDown,
                'F' => Pipe::RightDown,
                _ => Pipe::Ground,
            };
            if pipe == Pipe::Start {
                start = (x as i32, y as i32);
            }
            map.insert((x as i32, y as i32), pipe);
        }
    }
    if start == (0, 0) {
        panic!("No start found");
    }

    let walker = |current: &(i32, i32), last: &(i32, i32)| {
        let pipe = map.get(&current).unwrap();

        let mut ret = current.clone();

        match pipe {
            //
            Pipe::TopDown => {
                if current.1 > last.1 {
                    ret.1 += 1;
                } else {
                    ret.1 -= 1;
                }
            }
            Pipe::LeftRight => {
                if current.0 > last.0 {
                    ret.0 += 1;
                } else {
                    ret.0 -= 1;
                }
            }
            Pipe::TopRight => {
                if current.1 > last.1 {
                    ret.0 += 1;
                } else {
                    ret.1 -= 1;
                }
            }
            Pipe::TopLeft => {
                if current.1 > last.1 {
                    ret.0 -= 1;
                } else {
                    ret.1 -= 1;
                }
            }
            Pipe::LeftDown => {
                if current.0 > last.0 {
                    ret.1 += 1;
                } else {
                    ret.0 -= 1;
                }
            }
            Pipe::RightDown => {
                if current.0 < last.0 {
                    ret.1 += 1;
                } else {
                    ret.0 += 1;
                }
            }

            _ => {}
        }
        ret
    };
    let mut steps: usize = 1;

    let mut branch1_last = start;
    let mut branch2_last = start;
    let mut branch1: Option<(i32, i32)> = None;
    let mut branch2: Option<(i32, i32)> = None;

    // above
    let above = map.get(&(start.0, start.1 - 1)).unwrap();
    if above == &Pipe::TopDown || above == &Pipe::LeftDown || above == &Pipe::RightDown {
        if branch1.is_none() {
            branch1 = Some((start.0, start.1 - 1));
        } else {
            branch2 = Some((start.0, start.1 - 1));
        }
    }

    let left = map.get(&(start.0 - 1, start.1)).unwrap();
    if left == &Pipe::LeftRight || left == &Pipe::TopRight || left == &Pipe::RightDown {
        if branch1.is_none() {
            branch1 = Some((start.0 - 1, start.1));
        } else {
            branch2 = Some((start.0 - 1, start.1));
        }
    }

    let right = map.get(&(start.0 + 1, start.1)).unwrap();
    if right == &Pipe::LeftRight || right == &Pipe::TopLeft || right == &Pipe::LeftDown {
        if branch1.is_none() {
            branch1 = Some((start.0 + 1, start.1));
        } else {
            branch2 = Some((start.0 + 1, start.1));
        }
    }

    let bottom = map.get(&(start.0, start.1 + 1)).unwrap();
    if bottom == &Pipe::TopDown || bottom == &Pipe::TopRight || bottom == &Pipe::TopLeft {
        if branch1.is_none() {
            branch1 = Some((start.0, start.1 + 1));
        } else {
            branch2 = Some((start.0, start.1 + 1));
        }
    }
    // dbg!(branch1, branch2);

    loop {
        let mut tmp = branch1;
        branch1 = Some(walker(&branch1.unwrap(), &branch1_last));
        branch1_last = tmp.unwrap();

        // println!("{:?} -> {:?}", branch1_last, branch1);

        tmp = branch2;
        branch2 = Some(walker(&branch2.unwrap(), &branch2_last));
        branch2_last = tmp.unwrap();

        steps += 1;

        if branch2 == branch1 {
            break;
        }
    }

    dbg!(steps);
}
