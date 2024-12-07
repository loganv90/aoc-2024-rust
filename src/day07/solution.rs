use std::fs;
use std::env;
use crate::Day;

pub struct Day07 {}

impl Day for Day07 {
    fn execute(&self) {
        println!("Day07");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day07/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day07/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut eqs: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in lines {
        let line_split = line
            .split([':', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let total = line_split[0];
        let afters = line_split[1..].to_vec();
        eqs.push((total, afters));
    }

    let mut total = 0;
    for eq in eqs {
        if equal1(&eq, eq.1[0], 1) {
            total += eq.0;
        }
    }

    println!("Part 1: {}", total);
}

fn equal1(eq: &(i64, Vec<i64>), total: i64, index: i64) -> bool {
    if index >= eq.1.len() as i64 {
        return total == eq.0;
    }

    let res1 = equal1(eq, total + eq.1[index as usize], index + 1);
    let res2 = equal1(eq, total * eq.1[index as usize], index + 1);
    return res1 || res2;
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut eqs: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in lines {
        let line_split = line
            .split([':', ' '])
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let total = line_split[0];
        let afters = line_split[1..].to_vec();
        eqs.push((total, afters));
    }

    let mut total = 0;
    for eq in eqs {
        if equal2(&eq, eq.1[0], 1) {
            total += eq.0;
        }
    }

    println!("Part 2: {}", total);
}

fn equal2(eq: &(i64, Vec<i64>), total: i64, index: i64) -> bool {
    if index >= eq.1.len() as i64 {
        return total == eq.0;
    }

    let res1 = equal2(eq, total + eq.1[index as usize], index + 1);
    let res2 = equal2(eq, total * eq.1[index as usize], index + 1);

    let s1 = total.to_string();
    let s2 = eq.1[index as usize].to_string();
    let num = (s1 + &s2).parse::<i64>().unwrap();
    let res3 = equal2(eq, num, index + 1);

    return res1 || res2 || res3;
}

