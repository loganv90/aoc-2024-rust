use std::fs;
use std::env;
use std::collections::HashMap;
use crate::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn execute(&self) {
        println!("Day01");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day01/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day01/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in lines {
        let line_split = line.split(" ").filter(|x| x.len() > 0).collect::<Vec<&str>>();
        if line_split.len() != 2 {
            panic!("Invalid input");
        }

        let left_num = line_split[0].parse::<i64>().unwrap();
        left_list.push(left_num);

        let right_num = line_split[1].parse::<i64>().unwrap();
        right_list.push(right_num);
    }

    left_list.sort();
    right_list.sort();

    let mut diff = 0;
    for i in 0..left_list.len() {
        let left_num = left_list[i];
        let right_num = right_list[i];

        diff += (left_num - right_num).abs();
    }

    println!("Part 1: {}", diff);
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut left_list = vec![];
    let mut right_map = HashMap::new();

    for line in lines {
        let line_split = line.split(" ").filter(|x| x.len() > 0).collect::<Vec<&str>>();
        if line_split.len() != 2 {
            panic!("Invalid input");
        }

        let left_num = line_split[0].parse::<i64>().unwrap();
        left_list.push(left_num);

        let right_num = line_split[1].parse::<i64>().unwrap();
        match right_map.get(&right_num) {
            Some(cur) => { right_map.insert(right_num, cur + 1); },
            None => { right_map.insert(right_num, 1); },
        }
    }

    let mut res = 0;
    for left in left_list {
        res += right_map.get(&left).unwrap_or(&0) * left;
    }

    println!("Part 2: {}", res);
}

