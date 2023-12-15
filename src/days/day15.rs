use std::collections::HashMap;

use crate::utils;

#[derive(Debug)]
enum Action {
    Remove,
    Add(usize),
}

#[derive(Debug)]
struct Lens {
    label: String,
    power: usize,
}

pub fn part2() -> usize {
    let lines = utils::read_lines2("input/day15.txt");
    let lines = lines.strip_suffix("\n").unwrap();
    // let lines = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let steps = lines.split(",").collect::<Vec<_>>();

    let mut hashmap: HashMap<usize, Vec<Lens>> = HashMap::new();

    for step in steps {
        let label;
        let last = step.chars().last().unwrap();
        let action: Action;
        if last == '-' {
            label = step.chars().take(step.len() - 1).collect::<String>();
            action = Action::Remove;
        } else {
            label = step.chars().take(step.len() - 2).collect::<String>();
            action = Action::Add(last.to_digit(10).unwrap() as usize);
        }
        let boxnum = hash(&label);

        match action {
            Action::Add(power) => {
                let entry = hashmap.entry(boxnum).or_insert(vec![]);

                let mut found = false;
                for i in entry.iter_mut() {
                    if i.label == label {
                        found = true;
                        i.power = power;
                        break;
                    }
                }
                if !found {
                    entry.push(Lens { label, power });
                }
            }
            Action::Remove => {
                let entry = hashmap.entry(boxnum).or_insert(vec![]);
                let mut found = None;
                for (i, elem) in entry.iter_mut().enumerate() {
                    if elem.label == label {
                        found = Some(i);
                        break;
                    }
                }
                if let Some(i) = found {
                    entry.remove(i);
                }
            }
        }
    }

    // calculate power
    let res = hashmap
        .iter()
        .map(|(boxnum, lensvec)| {
            let mut power = 0;
            for (i, val) in lensvec.iter().enumerate() {
                power += (boxnum + 1) * (i + 1) * val.power;
            }
            power
        })
        .sum::<usize>();

    res
}

pub fn part1() -> usize {
    let lines = utils::read_lines2("input/day15.txt");
    let lines = lines.strip_suffix("\n").unwrap();

    let ret = lines.split(",").map(|x| hash(x)).sum::<usize>();

    ret
}

fn hash(input: &str) -> usize {
    let mut output = 0;
    for c in input.chars() {
        output += c as usize;
        output *= 17;
        output = output % 256;
    }
    output
}
