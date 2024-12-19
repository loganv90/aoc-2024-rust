use std::fs;
use std::env;
use crate::Day;

pub struct Day19 {}

impl Day for Day19 {
    fn execute(&self) {
        println!("Day19");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day19/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day19/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut line = lines.next();
    let towels = line.unwrap().split(", ").collect::<Vec<&str>>();

    let mut patterns: Vec<&str> = Vec::new();
    lines.next();
    line = lines.next();
    while line.is_some() {
        patterns.push(line.unwrap());
        line = lines.next();
    }

    let mut total = 0;
    for pattern in &patterns {
        if pattern_match(&towels, pattern) {
            total += 1;
        }
    }

    println!("Part 1: {}", total);
}

fn pattern_match(towels: &Vec<&str>, pattern: &str) -> bool {
    let mut visited = vec![false; pattern.len() + 1];
    visited[0] = true;

    for i in 0..pattern.len() {
        if !visited[i] {
            continue;
        }

        for towel in towels {
            let end = i + towel.len();

            if end > pattern.len() {
                continue;
            }
            
            if &pattern[i..end] != *towel {
                continue
            }

            visited[end] = true;
        }
    }

    return visited[pattern.len()];
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let mut line = lines.next();
    let towels = line.unwrap().split(", ").collect::<Vec<&str>>();

    let mut patterns: Vec<&str> = Vec::new();
    lines.next();
    line = lines.next();
    while line.is_some() {
        patterns.push(line.unwrap());
        line = lines.next();
    }

    let mut total = 0;
    for pattern in &patterns {
        total += pattern_match2(&towels, pattern);
    }

    println!("Part 2: {}", total);
}

fn pattern_match2(towels: &Vec<&str>, pattern: &str) -> i64 {
    let mut visited = vec![0; pattern.len() + 1];
    visited[0] = 1;

    for i in 0..pattern.len() {
        if visited[i] < 1 {
            continue;
        }

        for towel in towels {
            let end = i + towel.len();

            if end > pattern.len() {
                continue;
            }
            
            if &pattern[i..end] != *towel {
                continue
            }

            visited[end] += visited[i];
        }
    }

    return visited[pattern.len()];
}

