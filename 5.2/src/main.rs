use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;
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

#[derive(Debug)]
struct Mapping {
    /* ( output_soil, input_seed, len ) */
    // they are sorted by input_start
    list: Vec<(isize, isize, isize)>,
}

#[derive(Debug, PartialEq)]
struct Range {
    start: isize,
    len: isize,
}

impl Range {
    fn apply_mapping(self: &Self, mapping: &Mapping) -> Vec<Range> {
        let mut result = Vec::new();
        let mut self_start = self.start;
        let self_end = self.start + self.len - 1;

        for m in &mapping.list {
            let mapping_start = m.1;
            let mapping_end = m.1 + m.2 - 1;
            // dbg!(self_start, self_end, mapping_start, mapping_end);

            if mapping_end < self_start {
                continue;
            }
            if mapping_start > self_end {
                break;
            }

            // before
            if self_start < mapping_start {
                result.push(Range {
                    start: self_start,
                    len: mapping_start - self_start,
                });
                self_start = mapping_start;
            }
            if self_end >= mapping_end {
                result.push(Range {
                    start: self_start - mapping_start + m.0,
                    len: mapping_end - self_start + 1,
                });
                self_start = mapping_end + 1;
            } else {
                // dbg!(self_start, self_end, mapping_start, mapping_end);
                if self_end - self_start >= 0 {
                    result.push(Range {
                        start: self_start - mapping_start + m.0,
                        len: self_end - self_start + 1,
                    });
                    self_start = self_end + 1;
                }
            }
        }
        if self_start <= self_end {
            result.push(Range {
                start: self_start,
                len: self_end - self_start + 1,
            });
        }
        // deoverlap ranges
        // result = deoverlap_ranges(result);
        return result;
    }
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

    let seeds = lines[0].splitn(2, " ").collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|x| x.parse::<isize>().unwrap())
        .tuples()
        .map(|(a, b)| Range { start: a, len: b })
        .collect::<Vec<Range>>();

    let mut mappings: Vec<Mapping> = Vec::new();

    for line in lines.iter().skip(2) {
        if line.is_empty() {
            continue;
        }

        if line.ends_with(":") {
            mappings.push(Mapping { list: Vec::new() });
            continue;
        }

        let numbers: (isize, isize, isize) = line
            .split(" ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect_tuple()
            .unwrap();

        mappings.last_mut().unwrap().list.push(numbers);
    }

    for mapping in mappings.iter_mut() {
        mapping.list.sort_by(|a, b| a.1.cmp(&b.1));
    }

    let mut new_ranges = seeds;
    for mapping in mappings.iter() {
        let mut current: Vec<Range> = Vec::new();

        for seed in new_ranges.iter() {
            let mut new_ranges = seed.apply_mapping(mapping);
            current.append(&mut new_ranges);
        }

        new_ranges = current;
    }

    let result = new_ranges.iter().min_by_key(|x| x.start).unwrap();
    println!("result = {}", result.start);
}
