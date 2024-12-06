use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::Day;

pub struct Day05 {}

impl Day for Day05 {
    fn execute(&self) {
        println!("Day05");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day05/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day05/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut num_to_befores = HashMap::<i64,Vec<i64>>::new();

    let mut line = lines.next();
    while !line.unwrap().is_empty() {
        let line_split = line.unwrap().split("|").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if line_split.len() != 2 {
            panic!("Invalid input");
        }

        match num_to_befores.get_mut(&line_split[0]) {
            Some(cur) => { cur.push(line_split[1]); },
            None => { num_to_befores.insert(line_split[0], vec![line_split[1]]); },
        }

        line = lines.next();
    }

    let mut total = 0;

    line = lines.next();
    while line.is_some() {
        let line_split = line.unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if order_valid(&num_to_befores, &line_split) {
            let mid = line_split.len() / 2;
            total += line_split[mid];
        }

        line = lines.next();
    }

    println!("Part 1: {}", total);
}

fn order_valid(num_to_befores: &HashMap<i64,Vec<i64>>, order: &Vec<i64>) -> bool {
    let mut visited = HashSet::<i64>::new();

    for num in order {
        let befores = num_to_befores.get(&num);
        if befores.is_none() {
            visited.insert(*num);
            continue;
        }

        for before in befores.unwrap() {
            if visited.contains(before) {
                return false;
            }
        }

        visited.insert(*num);
    }

    return true;
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let mut num_to_allows = HashMap::<i64,Vec<i64>>::new();

    let mut line = lines.next();
    while !line.unwrap().is_empty() {
        let line_split = line.unwrap().split("|").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        if line_split.len() != 2 {
            panic!("Invalid input");
        }

        match num_to_allows.get_mut(&line_split[0]) {
            Some(cur) => { cur.push(line_split[1]); },
            None => { num_to_allows.insert(line_split[0], vec![line_split[1]]); },
        }

        line = lines.next();
    }

    let mut total = 0;

    line = lines.next();
    while line.is_some() {
        let line_split = line.unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if !order_valid(&num_to_allows, &line_split) {
            let order = generate_order(&num_to_allows, &line_split);
            let mid = order.len() / 2;
            total += order[mid];
        }

        line = lines.next();
    }

    println!("Part 2: {}", total);
}

fn generate_order(num_to_allows: &HashMap<i64,Vec<i64>>, nums: &Vec<i64>) -> Vec<i64> {
    let mut res = Vec::<i64>::new();

    let mut num_to_counts = HashMap::<i64,i64>::new();
    for num in nums {
        for allow in num_to_allows.get(&num).unwrap_or(&vec![]) {
            match num_to_counts.get(allow) {
                Some(cur) => { num_to_counts.insert(*allow, cur + 1); },
                None => { num_to_counts.insert(*allow, 1); },
            }
        }
    }

    let mut q = VecDeque::<i64>::new();
    for num in nums {
        if num_to_counts.get(num).is_none() {
            q.push_back(*num);
        }
    }

    while q.len() > 0 {
        let num = q.pop_front().unwrap();
        if !nums.contains(&num) {
            continue;
        }

        for allow in num_to_allows.get(&num).unwrap_or(&vec![]) {
            let count = *num_to_counts.get(allow).unwrap();
            num_to_counts.insert(*allow, count - 1);
            if count - 1 == 0 {
                q.push_back(*allow);
            }
        }

        res.push(num);
    }

    return res;
}

