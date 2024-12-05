use std::fs;
use std::env;
use regex::Regex;
use crate::Day;

pub struct Day03 {}

impl Day for Day03 {
    fn execute(&self) {
        println!("Day03");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day03/test1");
        let mut test2_path = base_path.clone();
        test2_path.push("src/day03/test2");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day03/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test2_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut total = 0;
    for line in lines {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

        for capture in re.captures_iter(line) {
            let x = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            total += x * y;
        }
    }

    println!("Part 1: {}", total);
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut total = 0;
    let mut enabled = true;
    for line in lines {
        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|don't\(\)|do\(\)").unwrap();

        for capture in re.captures_iter(line) {
            if capture.get(0).unwrap().as_str() == "do()" {
                enabled = true;
                continue;
            } else if capture.get(0).unwrap().as_str() == "don't()" {
                enabled = false;
                continue;
            }

            if !enabled {
                continue;
            }

            let x = capture.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            total += x * y;
        }
    }

    println!("Part 1: {}", total);
}

