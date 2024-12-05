use std::fs;
use std::env;
use crate::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn execute(&self) {
        println!("Day02");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day02/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day02/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut safe_count = 0;
    for line in lines {
        let line_split = line
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        if safe(&line_split) {
            safe_count += 1;
        }
    }

    println!("Part 1: {}", safe_count);
}

fn safe(l: &Vec<i64>) -> bool {
    let increasing = l[0] < l[1];
    let mut prev = l[0];

    let mut i = 1;
    while i < l.len() {
        let curr = l[i];
        let diff = (prev - curr).abs();
        if increasing != (prev < curr) || diff > 3 || diff < 1 {
            return false;
        }
        prev = curr;
        i += 1;
    }

    return true;
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut safe_count = 0;
    for line in lines {
        let mut line_split = line
            .split(" ")
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let safe = remove_one_safe(&line_split);
        line_split.reverse();
        let safe2 = remove_one_safe(&line_split);

        if safe || safe2 {
            safe_count += 1;
        }
    }

    println!("Part 1: {}", safe_count);
}

fn remove_one_safe(l: &Vec<i64>) -> bool {
    let increasing = l[0] < l[1];
    let mut skip = false;
    let mut prev = l[0];

    let mut i = 1;
    while i < l.len() {
        let curr = l[i];
        let diff = (prev - curr).abs();
        if increasing != (prev < curr) || diff > 3 || diff < 1 {
            if skip {
                return false;
            }
            skip = true;
        } else {
            prev = curr;
        }
        i += 1;
    }

    return true;
}

