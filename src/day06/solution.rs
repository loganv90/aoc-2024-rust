use std::fs;
use std::env;
use std::collections::HashSet;
use crate::Day;

pub struct Day06 {}

impl Day for Day06 {
    fn execute(&self) {
        println!("Day06");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day06/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day06/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut cr: i64 = 0;
    let mut cc: i64 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (r, line) in lines.enumerate() {
        let mut row: Vec<char> = Vec::new();

        for (c, ch) in line.chars().enumerate() {
            if ch == '^' {
                cr = r as i64;
                cc = c as i64;
                row.push('X');
            } else {
                row.push(ch);
            }
        }

        grid.push(row);
    }

    let mut count = 1;
    let mut di = 0;
    let directions = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ];

    loop {
        let nr = cr + directions[di].0;
        let nc = cc + directions[di].1;
        if nr < 0 || nr >= grid.len() as i64 || nc < 0 || nc >= grid[0].len() as i64 {
            break;
        }

        let ch = grid[nr as usize][nc as usize];
        if ch == '#' {
            di = (di + 1) % 4;
            continue;
        } else if ch == '.' {
            count += 1;
            grid[nr as usize][nc as usize] = 'X';
        }

        cr = nr;
        cc = nc;
    }

    println!("Part 1: {}", count);
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut cr: i64 = 0;
    let mut cc: i64 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for (r, line) in lines.enumerate() {
        let mut row: Vec<char> = Vec::new();

        for (c, ch) in line.chars().enumerate() {
            if ch == '^' {
                cr = r as i64;
                cc = c as i64;
                row.push('X');
            } else {
                row.push(ch);
            }
        }

        grid.push(row);
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '.' {
                grid[r][c] = '#';

                if detect_loop(&grid, cr as i64, cc as i64) {
                    count += 1;
                }

                grid[r][c] = '.';
            }
        }
    }

    println!("Part 2: {}", count);
}

fn detect_loop(grid: &Vec<Vec<char>>, mut cr: i64, mut cc: i64) -> bool {
    let mut di = 0;
    let directions = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
    ];

    let mut visited: HashSet<(i64, i64, i64, i64)> = HashSet::new();

    loop {
        let nr = cr + directions[di].0;
        let nc = cc + directions[di].1;
        if nr < 0 || nr >= grid.len() as i64 || nc < 0 || nc >= grid[0].len() as i64 {
            break;
        }

        let ch = grid[nr as usize][nc as usize];
        if ch == '#' {
            let key = (nr, nc, cr, cc);
            if visited.contains(&key) {
                return true;
            }
            visited.insert(key);

            di = (di + 1) % 4;
            continue;
        }

        cr = nr;
        cc = nc;
    }

    return false;
}

