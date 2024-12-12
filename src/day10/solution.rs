use std::fs;
use std::env;
use std::collections::VecDeque;
use std::collections::HashSet;
use crate::Day;

pub struct Day10 {}

impl Day for Day10 {
    fn execute(&self) {
        println!("Day10");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day10/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day10/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut row: Vec<i32> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    let mut total = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, num) in row.iter().enumerate() {
            if num == &0 {
                total += score(&grid, r as i32, c as i32);
            }
        }
    }

    println!("Part 1: {}", total);
}

fn score(grid: &Vec<Vec<i32>>, r: i32, c: i32) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((r, c));

    let mut search = 1;
    let directions = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ];

    while q.len() > 0 {
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in &directions {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[nr as usize].len() as i32 {
                    continue;
                }

                let val = grid[nr as usize][nc as usize];
                if val != search {
                    continue;
                }

                q.push_back((nr, nc));
            }
        }

        search += 1;
        if search > 9 {
            break;
        }
    }

    return HashSet::<&(i32, i32)>::from_iter(q.iter()).len() as i32;
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut row: Vec<i32> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    let mut total = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, num) in row.iter().enumerate() {
            if num == &0 {
                total += score2(&grid, r as i32, c as i32);
            }
        }
    }

    println!("Part 2: {}", total);
}

fn score2(grid: &Vec<Vec<i32>>, r: i32, c: i32) -> i32 {
    let mut q = VecDeque::new();
    q.push_back((r, c));

    let mut search = 1;
    let directions = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ];

    while q.len() > 0 {
        for _ in 0..q.len() {
            let (r, c) = q.pop_front().unwrap();
            for (dr, dc) in &directions {
                let nr = r + dr;
                let nc = c + dc;
                if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[nr as usize].len() as i32 {
                    continue;
                }

                let val = grid[nr as usize][nc as usize];
                if val != search {
                    continue;
                }

                q.push_back((nr, nc));
            }
        }

        search += 1;
        if search > 9 {
            break;
        }
    }

    return q.iter().len() as i32;
}

