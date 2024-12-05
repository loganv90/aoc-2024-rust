use std::fs;
use std::env;
use crate::Day;

pub struct Day04 {}

impl Day for Day04 {
    fn execute(&self) {
        println!("Day04");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day04/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day04/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let directions = vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for direction in &directions {
                if find_word("XMAS", &grid, row as i64, col as i64, direction.0, direction.1) {
                    total += 1;
                }
            }
        }
    }

    println!("Part 1: {}", total);
}

fn find_word(word: &str, grid: &Vec<Vec<char>>, mut r: i64, mut c: i64, dr: i64, dc: i64) -> bool {
    let mut chars = word.chars();

    let mut ch = chars.next();
    while ch.is_some() && r >= 0 && r < grid.len() as i64 && c >= 0 && c < grid[r as usize].len() as i64 {
        if grid[r as usize][c as usize] != ch.unwrap() {
            return false;
        }

        r += dr;
        c += dc;
        ch = chars.next();
    }

    return ch.is_none();
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        let mut visited_row = Vec::new();
        for c in line.chars() {
            row.push(c);
            visited_row.push(false);
        }
        grid.push(row);
        visited.push(visited_row);
    }

    let directions = vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let mut total = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for direction in &directions {
                if find_word("MAS", &grid, row as i64, col as i64, direction.0, direction.1) {
                    let mid_row = (row as i64 + direction.0) as usize;
                    let mid_col = (col as i64 + direction.1) as usize;
                    if visited[mid_row][mid_col] {
                        total += 1;
                    } else {
                        visited[mid_row][mid_col] = true;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", total);
}

