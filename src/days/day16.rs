use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
struct Beam {
    start: (isize, isize),

    // 0 - up , 1 - right, 2 - down, 3 - left
    dir: isize,
}

#[derive(Debug)]
struct BeamTile {
    beam: (bool, bool, bool, bool),
}
impl BeamTile {
    fn energized(&self) -> bool {
        self.beam.0 || self.beam.1 || self.beam.2 || self.beam.3
    }
}

pub fn part2() -> isize {
    //     let lines = r".|...\....
    // |.-.\.....
    // .....|-...
    // ........|.
    // ..........
    // .........\
    // ..../.\\..
    // .-.-/..|..
    // .|....-|.\
    // ..//.|....";

    let lines = utils::read_lines2("input/day16.txt");

    let max_x = lines.split("\n").next().unwrap().len() as isize;
    let max_y = lines.split("\n").count() as isize;

    let mut max = 0;

    // left and right
    for i in 0..lines.split("\n").count() {
        let res = analyze(
            &lines,
            Beam {
                start: (-1, i as isize),
                dir: 1,
            },
        );
        if res > max {
            max = res;
        }

        let res = analyze(
            &lines,
            Beam {
                start: (max_x, i as isize),
                dir: 3,
            },
        );
        if res > max {
            max = res;
        }
        //
    }

    // top and bottom
    for i in 0..max_x {
        let res = analyze(
            &lines,
            Beam {
                start: (i as isize, -1),
                dir: 2,
            },
        );
        if res > max {
            max = res;
        }

        let res = analyze(
            &lines,
            Beam {
                start: (i as isize, max_y),
                dir: 0,
            },
        );
        if res > max {
            max = res;
        }
        //
    }

    max
}

pub fn part1() -> isize {
    //     let lines = r".|...\....
    // |.-.\.....
    // .....|-...
    // ........|.
    // ..........
    // .........\
    // ..../.\\..
    // .-.-/..|..
    // .|....-|.\
    // ..//.|....";

    let lines = utils::read_lines2("input/day16.txt");
    analyze(
        &lines,
        Beam {
            start: (-1, 0),
            dir: 1,
        },
    )
}

fn analyze(input: &str, beam: Beam) -> isize {
    let lines = input;

    let mut energize_map: HashMap<(isize, isize), BeamTile> = HashMap::new();
    let mut map = HashMap::new();

    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in lines.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            let y = y as isize;
            let x = x as isize;
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
            map.insert((x, y), c);
        }
    }

    let mut beams = vec![beam];

    loop {
        let beam = beams.pop();
        if beam.is_none() {
            break;
        }
        let beam = beam.unwrap();

        beams.append(&mut run_laser(&map, beam, &mut energize_map, max_x, max_y));
    }

    energize_map.iter().filter(|(_, v)| v.energized()).count() as isize
}

fn run_laser(
    map: &HashMap<(isize, isize), char>,
    mut beamhead: Beam,
    emap: &mut HashMap<(isize, isize), BeamTile>,
    max_x: isize,
    max_y: isize,
) -> Vec<Beam> {
    let mut split = Vec::new();
    loop {
        // advance beam
        match beamhead.dir {
            0 => beamhead.start.1 -= 1,
            1 => beamhead.start.0 += 1,
            2 => beamhead.start.1 += 1,
            3 => beamhead.start.0 -= 1,
            _ => panic!("invalid direction"),
        }

        // check if beam is out of bounds
        if beamhead.start.0 < 0
            || beamhead.start.1 < 0
            || beamhead.start.0 > max_x
            || beamhead.start.1 > max_y
        {
            break;
        }

        // check if beam is energized
        if emap.contains_key(&beamhead.start) {
            let tile = emap.get_mut(&beamhead.start).unwrap();
            if beamhead.dir == 0 && tile.beam.0 {
                break;
            }
            if beamhead.dir == 1 && tile.beam.1 {
                break;
            }
            if beamhead.dir == 2 && tile.beam.2 {
                break;
            }
            if beamhead.dir == 3 && tile.beam.3 {
                break;
            }
            match beamhead.dir {
                0 => tile.beam.0 = true,
                1 => tile.beam.1 = true,
                2 => tile.beam.2 = true,
                3 => tile.beam.3 = true,
                _ => panic!("invalid direction"),
            }
        } else {
            let mut tile = BeamTile {
                beam: (false, false, false, false),
            };
            match beamhead.dir {
                0 => tile.beam.0 = true,
                1 => tile.beam.1 = true,
                2 => tile.beam.2 = true,
                3 => tile.beam.3 = true,
                _ => panic!("invalid direction"),
            }
            emap.insert(beamhead.start, tile);
        }

        // find next direction
        let char = map.get(&beamhead.start).unwrap();

        match char {
            '.' => {}
            '|' => match beamhead.dir {
                0 => beamhead.dir = 0,
                1 => {
                    beamhead.dir = 2;
                    split.push(Beam {
                        start: beamhead.start,
                        dir: 0,
                    })
                }
                2 => beamhead.dir = 2,
                3 => {
                    beamhead.dir = 0;
                    split.push(Beam {
                        start: beamhead.start,
                        dir: 2,
                    })
                }
                _ => panic!("invalid direction"),
            },
            '-' => {
                match beamhead.dir {
                    0 => {
                        beamhead.dir = 1;
                        split.push(Beam {
                            start: beamhead.start,
                            dir: 3,
                        });
                    }
                    1 => beamhead.dir = 1,
                    2 => {
                        beamhead.dir = 3;
                        split.push(Beam {
                            start: beamhead.start,
                            dir: 1,
                        });
                    }
                    3 => beamhead.dir = 3,
                    _ => panic!("invalid direction"),
                }
                // TODO: additional beam
            }
            '/' => match beamhead.dir {
                0 => beamhead.dir = 1,
                1 => beamhead.dir = 0,
                2 => beamhead.dir = 3,
                3 => beamhead.dir = 2,
                _ => panic!("invalid direction"),
            },
            '\\' => match beamhead.dir {
                0 => beamhead.dir = 3,
                1 => beamhead.dir = 2,
                2 => beamhead.dir = 1,
                3 => beamhead.dir = 0,
                _ => panic!("invalid direction"),
            },
            _ => panic!("invalid char"),
        }

        // println!("beamhead = {:?} char = {}", beamhead, char);
    }

    split
}
