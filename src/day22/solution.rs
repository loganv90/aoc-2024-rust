use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn execute(&self) {
        println!("Day22");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day22/test1");
        let mut test2_path = base_path.clone();
        test2_path.push("src/day22/test2");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day22/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test2_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut nums = Vec::new();
    for line in lines {
        let num = line.parse::<i64>().unwrap();
        nums.push(num);
    }

    let mut total = 0;
    for num in nums {
        let mut new_num = num;
        for _ in 0..2000 {
            new_num = get_generated_num(new_num);
        }
        total += new_num;
    }

    println!("Part 1: {}", total);
}

fn get_generated_num(starting_num: i64) -> i64 {
    let mut current_num = starting_num;

    current_num = (current_num * 64) ^ current_num;
    current_num = current_num % 16777216;

    current_num = (current_num / 32) ^ current_num;
    current_num = current_num % 16777216;

    current_num = (current_num * 2048) ^ current_num;
    current_num = current_num % 16777216;

    return current_num;
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut nums = Vec::new();
    for line in lines {
        let num = line.parse::<i64>().unwrap();
        nums.push(num);
    }

    let mut global_sequences = HashMap::<(i64, i64, i64, i64), i64>::new();
    for num in nums {
        let mut new_num = num;
        let mut sequence = VecDeque::<i64>::new();
        let mut local_sequences = HashSet::<(i64, i64, i64, i64)>::new();
        let mut last_price = 0;

        for _ in 0..3 {
            new_num = get_generated_num(new_num);
            let price = new_num % 10;
            let diff = price - last_price;
            sequence.push_back(diff);
            last_price = price;
        }
        for _ in 0..1997 {
            new_num = get_generated_num(new_num);
            let price = new_num % 10;
            let diff = price - last_price;
            sequence.push_back(diff);
            last_price = price;

            let tuple = (sequence[0], sequence[1], sequence[2], sequence[3]);
            match local_sequences.get(&tuple) {
                Some(_) => { },
                None => {
                    local_sequences.insert(tuple);
                    match global_sequences.get_mut(&tuple) {
                        Some(cur) => {
                            *cur += price;
                        },
                        None => {
                            global_sequences.insert(tuple, price);
                        },
                    }
                },
            }

            sequence.pop_front();
        }
    }

    let mut max = 0;
    for (_, price) in global_sequences {
        if price > max {
            max = price;
        }
    }

    println!("Part 2: {}", max);
}

