use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Empty,
    Pipe,
    Flooded,
}

fn dbg_draw(map: &HashMap<(i32, i32), Pipe>) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;

    for (k, _) in map.iter() {
        if k.0 < minx {
            minx = k.0;
        }
        if k.0 > maxx {
            maxx = k.0;
        }
        if k.1 < miny {
            miny = k.1;
        }
        if k.1 > maxy {
            maxy = k.1;
        }
    }

    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            let pipe = map.get(&(x, y)).unwrap_or(&Pipe::Ground);
            match pipe {
                Pipe::LeftRight => print!("-"),
                Pipe::TopDown => print!("|"),
                Pipe::TopRight => print!("L"),
                Pipe::TopLeft => print!("J"),
                Pipe::LeftDown => print!("7"),
                Pipe::RightDown => print!("F"),
                Pipe::Start => print!("S"),
                _ => print!("."),
            }
        }
        println!();
    }
    println!();
}

fn dbg_bitmap(map: &HashMap<(i32, i32), Tile>) {
    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;

    for (k, _) in map.iter() {
        if k.0 < minx {
            minx = k.0;
        }
        if k.0 > maxx {
            maxx = k.0;
        }
        if k.1 < miny {
            miny = k.1;
        }
        if k.1 > maxy {
            maxy = k.1;
        }
    }

    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            let pipe = map.get(&(x, y)).unwrap_or(&Tile::Empty);
            match pipe {
                Tile::Pipe => print!("P"),
                Tile::Flooded => print!("~"),
                _ => print!("."),
            }
        }
        println!();
    }
    println!();
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

    let mut steps: usize = 1;

    let mut branch1_last = start;
    let mut branch2_last = start;
    let mut branch1: Option<(i32, i32)> = None;
    let mut branch2: Option<(i32, i32)> = None;

    let (mut above_test, mut left_test, mut right_test, mut bottom_test) =
        (false, false, false, false);

    {
        // above
        let above = map.get(&(start.0, start.1 - 1)).unwrap();
        if above == &Pipe::TopDown || above == &Pipe::LeftDown || above == &Pipe::RightDown {
            above_test = true;
            if branch1.is_none() {
                branch1 = Some((start.0, start.1 - 1));
            } else {
                branch2 = Some((start.0, start.1 - 1));
            }
        }

        let left = map.get(&(start.0 - 1, start.1)).unwrap();
        if left == &Pipe::LeftRight || left == &Pipe::TopRight || left == &Pipe::RightDown {
            left_test = true;
            if branch1.is_none() {
                branch1 = Some((start.0 - 1, start.1));
            } else {
                branch2 = Some((start.0 - 1, start.1));
            }
        }

        let right = map.get(&(start.0 + 1, start.1)).unwrap();
        if right == &Pipe::LeftRight || right == &Pipe::TopLeft || right == &Pipe::LeftDown {
            right_test = true;
            if branch1.is_none() {
                branch1 = Some((start.0 + 1, start.1));
            } else {
                branch2 = Some((start.0 + 1, start.1));
            }
        }

        let bottom = map.get(&(start.0, start.1 + 1)).unwrap();
        if bottom == &Pipe::TopDown || bottom == &Pipe::TopRight || bottom == &Pipe::TopLeft {
            bottom_test = true;
            if branch1.is_none() {
                branch1 = Some((start.0, start.1 + 1));
            } else {
                branch2 = Some((start.0, start.1 + 1));
            }
        }
    }
    match (above_test, left_test, right_test, bottom_test) {
        (true, true, false, false) => {
            map.insert(start, Pipe::TopLeft);
        }
        (true, false, true, false) => {
            map.insert(start, Pipe::TopRight);
        }
        (false, true, true, false) => {
            map.insert(start, Pipe::LeftRight);
        }
        (false, false, true, true) => {
            map.insert(start, Pipe::RightDown);
        }
        _ => panic!("should not happen"),
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

    // dbg!(branch1, branch2);
    // init with all Empty
    let mut bitmap: HashMap<(i32, i32), Tile> =
        map.par_iter().map(|(k, _)| (*k, Tile::Empty)).collect();

    bitmap.insert(start, Tile::Pipe);
    loop {
        bitmap.insert(branch1.unwrap(), Tile::Pipe);
        bitmap.insert(branch2.unwrap(), Tile::Pipe);
        let mut tmp = branch1;
        branch1 = Some(walker(&branch1.unwrap(), &branch1_last));
        branch1_last = tmp.unwrap();

        // println!("{:?} -> {:?}", branch1_last, branch1);

        tmp = branch2;
        branch2 = Some(walker(&branch2.unwrap(), &branch2_last));
        branch2_last = tmp.unwrap();

        steps += 1;
        bitmap.insert(branch1.unwrap(), Tile::Pipe);
        bitmap.insert(branch2.unwrap(), Tile::Pipe);

        if branch2 == branch1 {
            break;
        }
    }
    // dbg_draw(&map);
    // dbg_bitmap(&bitmap);

    for y in 0..lines.len() {
        let mut pipes_passed = 0;
        let mut started_bottom = false;
        for x in 0..lines[0].len() {
            let val = bitmap.get(&(x as i32, y as i32)).unwrap_or(&Tile::Empty);
            if val == &Tile::Pipe {
                let val2 = map.get(&(x as i32, y as i32)).unwrap();

                match val2 {
                    Pipe::RightDown => {
                        started_bottom = true;
                    }
                    Pipe::TopRight => {
                        started_bottom = false;
                    }
                    Pipe::TopLeft => {
                        if started_bottom {
                            pipes_passed += 1;
                        }
                    }
                    Pipe::LeftDown => {
                        if !started_bottom {
                            pipes_passed += 1;
                        }
                    }
                    Pipe::TopDown => {
                        pipes_passed += 1;
                    }
                    _ => {}
                }
            }
            if val == &Tile::Empty && pipes_passed % 2 == 1 {
                bitmap.insert((x as i32, y as i32), Tile::Flooded);
            }
        }
    }

    dbg_bitmap(&bitmap);
    let res = bitmap.iter().filter(|(_, v)| v == &&Tile::Flooded).count();
    dbg!(res);

    // dbg!(path);
}
