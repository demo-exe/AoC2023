use crate::utils;

#[allow(dead_code)]
fn dbg(s: &Vec<String>) {
    for i in s {
        println!("{}", i);
    }
}

pub fn part1() -> isize {
    let lines = utils::read_lines("input/day11.txt");

    // this expansion step could be done cheaper in terms of memory or time but im lazy
    let mut lines_adjust1 = Vec::with_capacity(lines.len());
    let mut collumn_empty: Vec<bool> = lines[0].chars().map(|x| x != '#').collect();

    for i in lines.iter() {
        let mut empty = true;
        for (j, c) in i.chars().enumerate() {
            if c == '#' {
                empty = false;
                collumn_empty[j] = false;
            }
        }
        if empty {
            lines_adjust1.push(".".repeat(i.len()));
        }
        lines_adjust1.push(i.to_string());
    }

    let mut lines_adjust2 = Vec::with_capacity(lines_adjust1.len());

    let newlen = lines_adjust1[0].len() + collumn_empty.iter().filter(|x| **x).count();

    for line in lines_adjust1.iter() {
        let mut newline = String::with_capacity(newlen);
        for (i, shouldadd) in collumn_empty.iter().enumerate() {
            if *shouldadd {
                newline.push('.');
            }
            newline.push(line.chars().nth(i).unwrap());
        }
        lines_adjust2.push(newline);
    }

    // (x, y)
    let mut stars = Vec::new();

    for (i, line) in lines_adjust2.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                stars.push((j as i32, i as i32));
            }
        }
    }

    let mut sum = 0;

    for (i, star1) in stars.iter().take(stars.len() - 1).enumerate() {
        for star2 in stars.iter().skip(i) {
            let len = (star1.0 - star2.0).abs() + (star1.1 - star2.1).abs();
            sum += len;
        }
    }
    sum as isize
}

const SPACE_MULTIPLIER: i64 = 1000000 - 1;

pub fn part2() -> isize {
    let lines = utils::read_lines("input/day11.txt");

    // this expansion step could be done cheaper in terms of memory or time but im lazy
    // UPDATE: damn, they made me do it :/
    let mut collumn_empty: Vec<bool> = lines[0].chars().map(|x| x != '#').collect();
    let mut rows_empty: Vec<bool> = Vec::with_capacity(lines.len());

    for i in lines.iter() {
        let mut empty = true;
        for (j, c) in i.chars().enumerate() {
            if c == '#' {
                empty = false;
                collumn_empty[j] = false;
            }
        }
        if empty {
            rows_empty.push(true);
        } else {
            rows_empty.push(false);
        }
    }

    // (x, y)
    let mut stars = Vec::new();

    let mut row_adjust = 0;
    for (i, line) in lines.iter().enumerate() {
        if rows_empty[i] {
            row_adjust += 1;
        }
        let mut collumn_adjust = 0;
        for (j, c) in line.chars().enumerate() {
            if collumn_empty[j] {
                collumn_adjust += 1;
            }
            if c == '#' {
                stars.push((
                    (j as i64) + collumn_adjust * SPACE_MULTIPLIER,
                    (i as i64) + row_adjust * SPACE_MULTIPLIER,
                ));
            }
        }
    }
    let mut sum = 0;

    for (i, star1) in stars.iter().take(stars.len() - 1).enumerate() {
        for star2 in stars.iter().skip(i) {
            let len = (star1.0 - star2.0).abs() + (star1.1 - star2.1).abs();
            sum += len;
        }
    }
    sum as isize
}
