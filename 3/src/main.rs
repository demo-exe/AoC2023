use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let rightmax = lines[0].len() - 1;

    let analyze_box = |i: usize, pos: usize| {
        let topleft: Option<(usize, usize)>;
        let top: Option<(usize, usize)>;
        let topright: Option<(usize, usize)>;
        let left: Option<(usize, usize)>;
        let right: Option<(usize, usize)>;
        let bottomleft: Option<(usize, usize)>;
        let bottom: Option<(usize, usize)>;
        let bottomright: Option<(usize, usize)>;

        topleft = if i > 0 && pos > 0 {
            if lines[i - 1].chars().nth(pos - 1).unwrap().is_numeric() {
                Some((i - 1, pos - 1))
            } else {
                None
            }
        } else {
            None
        };

        top = if i > 0 {
            if lines[i - 1].chars().nth(pos).unwrap().is_numeric() {
                Some((i - 1, pos))
            } else {
                None
            }
        } else {
            None
        };

        topright = if i > 0 && pos < lines[i].len() - 1 {
            if lines[i - 1].chars().nth(pos + 1).unwrap().is_numeric() {
                Some((i - 1, pos + 1))
            } else {
                None
            }
        } else {
            None
        };

        left = if pos > 0 {
            if lines[i].chars().nth(pos - 1).unwrap().is_numeric() {
                Some((i, pos - 1))
            } else {
                None
            }
        } else {
            None
        };

        right = if pos < lines[i].len() - 1 {
            if lines[i].chars().nth(pos + 1).unwrap().is_numeric() {
                Some((i, pos + 1))
            } else {
                None
            }
        } else {
            None
        };

        bottomleft = if i < lines.len() - 1 && pos > 0 {
            if lines[i + 1].chars().nth(pos - 1).unwrap().is_numeric() {
                Some((i + 1, pos - 1))
            } else {
                None
            }
        } else {
            None
        };

        bottom = if i < lines.len() - 1 {
            if lines[i + 1].chars().nth(pos).unwrap().is_numeric() {
                Some((i + 1, pos))
            } else {
                None
            }
        } else {
            None
        };

        bottomright = if i < lines.len() - 1 && pos < lines[i].len() - 1 {
            if lines[i + 1].chars().nth(pos + 1).unwrap().is_numeric() {
                Some((i + 1, pos + 1))
            } else {
                None
            }
        } else {
            None
        };

        let all = vec![
            topleft,
            top,
            topright,
            left,
            right,
            bottomleft,
            bottom,
            bottomright,
        ];

        let mut res = Vec::with_capacity(8);
        for i in all {
            if let Some(b) = i {
                res.push(b);
            }
        }
        res
    };

    let mut hits: HashMap<(usize, usize), bool> = HashMap::new();

    lines.iter().enumerate().for_each(|(i, line)| {
        // find first matching char
        let _ = line
            .chars()
            .enumerate()
            .filter(|&(_, c)| c != '.' && !c.is_numeric())
            .for_each(|(pos, _a)| {
                // if i != 1 || pos != 74 {
                //     return;
                // }
                // dbg!((i, pos, _a));
                let res = analyze_box(i, pos);
                // dbg!(&res);
                for i in res {
                    hits.insert(i, false);
                }
            });
    });

    let mut asum = 0;
    'outer: for hit in hits.clone().iter_mut() {
        let i = hit.0 .0;
        let mut pos = hit.0 .1;

        loop {
            if pos == rightmax {
                break;
            }
            if lines[i].chars().nth(pos + 1).unwrap().is_numeric() {
                pos += 1;
            } else {
                break;
            }
        }

        let mut sum = 0;
        let mut exp = 1;

        loop {
            if pos == 0 {
                break;
            }

            if lines[i].chars().nth(pos).unwrap().is_numeric() {
                sum += exp * lines[i].chars().nth(pos).unwrap().to_digit(10).unwrap();
                let visited = hits.entry((i, pos)).or_insert(false);
                if *visited {
                    continue 'outer;
                }
                *visited = true;
                pos -= 1;
                exp *= 10;
            } else {
                break;
            }
        }
        asum += sum;
        // dbg!((i, pos, sum));
    }

    println!("result = {}", asum);

    // dbg!(hits);
}
