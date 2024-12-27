use std::fs;
use std::env;
use crate::Day;

pub struct Day25 {}

impl Day for Day25 {
    fn execute(&self) {
        println!("Day25");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day25/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day25/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut locks = Vec::<Vec<i64>>::new();
    let mut keys = Vec::<Vec<i64>>::new();

    loop {
        let top_line = lines.next().unwrap().chars().collect::<Vec<char>>();
        let grid = Vec::<Vec<char>>::from([
            lines.next().unwrap().chars().collect::<Vec<char>>(),
            lines.next().unwrap().chars().collect::<Vec<char>>(),
            lines.next().unwrap().chars().collect::<Vec<char>>(),
            lines.next().unwrap().chars().collect::<Vec<char>>(),
            lines.next().unwrap().chars().collect::<Vec<char>>(),
        ]);
        lines.next();

        if top_line[0] == '#' {
            locks.push(get_values(grid));
        } else {
            keys.push(get_values(grid));
        }

        if lines.next().is_none() {
            break;
        }
    }

    let mut total = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if compare(&lock, &key) {
                total += 1;
            }
        }
    }

    println!("Part 1: {}", total);
}

fn compare(lock: &Vec<i64>, key: &Vec<i64>) -> bool {
    if lock.len() != key.len() {
        panic!("Lock and key must be the same length");
    }
    for i in 0..lock.len() {
        if lock[i] + key[i] > 5 {
            return false;
        }
    }
    return true;
}

fn get_values(grid: Vec<Vec<char>>) -> Vec<i64> {
    let mut values: Vec<i64> = vec![0 as i64; grid[0].len()];
    for row in grid.iter() {
        for (c, col) in row.iter().enumerate() {
            if *col == '#' {
                values[c] += 1;
            }
        }
    }
    return values;
}

