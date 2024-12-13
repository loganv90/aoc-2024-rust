use std::fs;
use std::env;
use std::collections::HashMap;
use crate::Day;

pub struct Day11 {}

impl Day for Day11 {
    fn execute(&self) {
        println!("Day11");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day11/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day11/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();
    let line = lines.next().unwrap();

    let mut stones = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    for _ in 0..25 {
        let mut next = Vec::new();

        for stone in stones {
            if stone == 0 {
                next.push(1);
                continue;
            }

            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let front = &s[..s.len() / 2];
                let back = &s[s.len() / 2..];
                next.push(front.parse::<i64>().unwrap());
                next.push(back.parse::<i64>().unwrap());
                continue;
            }

            next.push(stone * 2024);
        }

        stones = next;
    }

    println!("Part 1: {}", stones.len());
}

fn part2(contents: String) {
    let mut lines = contents.lines();
    let line = lines.next().unwrap();

    let stones_vector = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut stones_map = HashMap::<i64,i64>::new();
    for stone in stones_vector {
        match stones_map.get_mut(&stone) {
            Some(cur) => { *cur += 1; },
            None => { stones_map.insert(stone, 1); },
        }
    }

    for _ in 0..75 {
        let mut next = HashMap::<i64,i64>::new();

        for (stone, count) in stones_map.iter() {
            if *stone == 0 {
                match next.get_mut(&1) {
                    Some(cur) => { *cur += count; },
                    None => { next.insert(1, *count); },
                }
                continue;
            }

            let s = stone.to_string();
            if s.len() % 2 == 0 {
                let front = &s[..s.len() / 2].parse::<i64>().unwrap();
                let back = &s[s.len() / 2..].parse::<i64>().unwrap();
                match next.get_mut(front) {
                    Some(cur) => { *cur += count; },
                    None => { next.insert(*front, *count); },
                }
                match next.get_mut(back) {
                    Some(cur) => { *cur += count; },
                    None => { next.insert(*back, *count); },
                }
                continue;
            }

            match next.get_mut(&(stone * 2024)) {
                Some(cur) => { *cur += count; },
                None => { next.insert(*stone * 2024, *count); },
            }
        }

        stones_map = next;
    }

    let mut total = 0;
    for (_, count) in stones_map.iter() {
        total += count;
    }

    println!("Part 2: {}", total);

}

