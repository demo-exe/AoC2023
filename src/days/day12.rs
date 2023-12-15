// absolutely dirtiest code I've ever written

use std::collections::HashMap;

use itertools::Itertools;

use crate::utils;

pub fn part2() -> isize {
    let mut lines = utils::read_lines("input/day12.txt");

    //     let mut lines = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1"
    //         .split("\n")
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();

    for line in lines.iter_mut() {
        let (block, groups) = line.split(" ").collect_tuple().unwrap();
        let mut newline = String::new();
        for _ in 0..4 {
            newline.push_str(block);
            newline.push_str("?");
        }
        newline.push_str(block);
        newline.push_str(" ");
        for _ in 0..4 {
            newline.push_str(groups);
            newline.push_str(",");
        }
        newline.push_str(groups);
        *line = newline;
    }

    // dbg!(&lines);

    // analyze(&lines[0]);
    //
    // analyze(&lines[5]);
    // dbg!(analyze(&String::from("##?#? 3,1")));
    //
    let result = lines.iter().map(|x| analyze(x)).sum::<usize>();

    result as isize
}

pub fn part1() -> isize {
    let lines = utils::read_lines("input/day12.txt");

    //     let mut lines = "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1"
    //         .split("\n")
    //         .map(|x| x.to_string())
    //         .collect::<Vec<String>>();

    // analyze(&lines[0]);
    //
    // analyze(&lines[5]);
    // dbg!(analyze(&String::from("##?#? 3,1")));
    //
    let result = lines.iter().map(|x| analyze(x)).sum::<usize>();
    result as isize
}

fn analyze(line: &String) -> usize {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let block = parts[0];

    let groups = parts[1]
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // dbg!(block, groups);
    let blocks = block.chars().collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let ret = count_possibilities(&mut cache, &blocks, &groups, 0);

    ret
}

#[allow(dead_code)]
fn print(line: String, depth: u32) {
    for _ in 0..depth {
        print!(">>");
    }
    println!(" {}", line);
}

fn count_possibilities<'a>(
    cache: &mut HashMap<(&'a [char], &'a [usize]), usize>,
    mut block: &'a [char],
    groups: &'a [usize],
    depth: usize,
) -> usize {
    // trim leading .s
    while let ['.', rest @ ..] = block {
        block = rest;
    }
    // println!("blocks = {:?} groups = {:?}", block, groups);
    // print(format!("blocks = {:?} groups = {:?}", block, groups), depth);

    // stop coditions
    if block.len() == 0 && groups.len() == 0 {
        return 1;
    }
    if block.len() == 0 && groups.len() > 0 {
        return 0;
    }
    if block.len() > 0 && groups.len() == 0 {
        let mut all_dots = true;
        for i in 0..block.len() {
            if block[i] == '#' {
                all_dots = false;
                break;
            }
        }
        if all_dots {
            return 1;
        } else {
            return 0;
        }
    }

    // check cache
    if let Some(&ret) = cache.get(&(block, groups)) {
        return ret;
    }

    if block[0] == '#' {
        if groups[0] as usize > block.len() {
            return 0;
        }
        for i in 0..groups[0] {
            if block[i as usize] == '.' {
                return 0;
            }
        }
        if groups[0] as usize == block.len() && groups.len() == 1 {
            return 1;
        }
        if groups[0] as usize + 1 > block.len() {
            return 0;
        }
        if block[groups[0] as usize] == '#' {
            return 0;
        }
        // set cache
        let ret = count_possibilities(
            cache,
            &block[groups[0] as usize + 1..],
            &groups[1..],
            depth + 1,
        );
        cache.insert((block, groups), ret);
        return ret;
    } else {
        // must be a ? then
        // suppose its .
        let count_if_working = count_possibilities(cache, &block[1..], groups, depth + 1);
        // println!("count_if_working = {}", count_if_working);
        // print(format!("count_if_working = {}", count_if_working), depth);

        // suppose its #
        let mut possible_to_match_here = true;
        if groups[0] as usize > block.len() {
            //set cache
            cache.insert((block, groups), count_if_working);
            return count_if_working;
        }
        for i in 0..groups[0] {
            if block[i as usize] == '.' {
                possible_to_match_here = false;
                break;
            }
        }
        // dbg!(groups[0] as usize, block.len());
        if groups[0] as usize == block.len() {
            // do nothing, this may be last possible match
        } else if block[groups[0] as usize] == '#' {
            possible_to_match_here = false;
        }
        // print(
        //     format!("possible_to_match_here = {}", possible_to_match_here),
        //     depth,
        // );

        if possible_to_match_here {
            // consuming the rest
            if groups[0] as usize == block.len() && groups.len() == 1 {
                // set cache
                cache.insert((block, groups), count_if_working + 1);
                return count_if_working + 1;
            }

            if groups[0] as usize + 1 > block.len() {
                // set cache
                cache.insert((block, groups), count_if_working);
                return count_if_working;
            }

            let further_possibilities = count_possibilities(
                cache,
                &block[groups[0] as usize + 1..],
                &groups[1..],
                depth + 1,
            );

            // println!(
            //     "blocks = {:?} groups = {:?} possible = {}, further = {}",
            //     block, groups, possible_to_match_here, further_possibilities
            // );
            if further_possibilities == 0 {
                // print(
                //     format!("cutting short, count = {}", count_if_working),
                //     depth,
                // );
                // set cache
                cache.insert((block, groups), count_if_working);
                return count_if_working;
            } else {
                // print(
                //     format!(
                //         "block = {:?} groups = {:?} possible = {}",
                //         block, groups, possible_to_match_here,
                //     ),
                //     depth,
                // );
                // print(
                //     format!(
                //         "was possible, count = {}. further = {}",
                //         count_if_working, further_possibilities
                //     ),
                //     depth,
                // );
                // set cache
                cache.insert((block, groups), count_if_working + further_possibilities);
                return count_if_working + further_possibilities;
            }
        } else {
            // println!("not possible");
            // print(format!("not possible"), depth);
            return count_if_working;
        }
    }
}
