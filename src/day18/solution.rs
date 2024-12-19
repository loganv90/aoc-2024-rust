use std::fs;
use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::Day;

pub struct Day18 {}

impl Day for Day18 {
    fn execute(&self) {
        println!("Day18");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day18/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day18/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut line = lines.next();
    let mut line_split = line.unwrap().split(",");
    let height = line_split.next().unwrap().strip_prefix("h=").unwrap().parse::<i64>().unwrap() + 1;
    let width = line_split.next().unwrap().strip_prefix("w=").unwrap().parse::<i64>().unwrap() + 1;
    let corrupt = line_split.next().unwrap().strip_prefix("c=").unwrap().parse::<i64>().unwrap();
    let mut positions: Vec<(i64, i64)> = Vec::new();
    line = lines.next();
    while line.is_some() {
        let line_split = line.unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        positions.push((line_split[0], line_split[1]));
        line = lines.next();
    }

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for i in 0..corrupt {
        let position = positions[i as usize];
        grid[position.1 as usize][position.0 as usize] = '#';
    }

    println!("Part 1: {}", bfs(&grid, 0, 0, height - 1, width - 1));
}

fn bfs(grid: &Vec<Vec<char>>, rs: i64, cs: i64, re: i64, ce: i64) -> i64 {
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut queue: VecDeque<(i64, i64, i64)> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    queue.push_back((rs, cs, 0));
    visited.insert((rs, cs));

    while !queue.is_empty() {
        let (rc, cc, d) = queue.pop_front().unwrap();
        if rc == re && cc == ce {
            return d;
        }

        for direction in &directions {
            let (rn, cn) = (rc + direction.0, cc + direction.1);
            if rn < 0 || rn >= grid.len() as i64 || cn < 0 || cn >= grid[rn as usize].len() as i64 {
                continue;
            }

            let ch = grid[rn as usize][cn as usize];
            if ch == '#' {
                continue;
            }

            if visited.contains(&(rn, cn)) {
                continue;
            }

            queue.push_back((rn, cn, d + 1));
            visited.insert((rn, cn));
        }
    }

    return -1;
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let mut line = lines.next();
    let mut line_split = line.unwrap().split(",");
    let height = line_split.next().unwrap().strip_prefix("h=").unwrap().parse::<i64>().unwrap() + 1;
    let width = line_split.next().unwrap().strip_prefix("w=").unwrap().parse::<i64>().unwrap() + 1;
    let mut positions: Vec<(i64, i64)> = Vec::new();
    line = lines.next();
    while line.is_some() {
        let line_split = line.unwrap().split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        positions.push((line_split[0], line_split[1]));
        line = lines.next();
    }

    let mut res = (-1, -1);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for position in &positions { // We're likely meant to binary search, but this is fast enough
        grid[position.1 as usize][position.0 as usize] = '#';

        let search_result = bfs(&grid, 0, 0, height - 1, width - 1);
        if search_result < 0 {
            res = (position.0, position.1);
            break;
        }
    }

    println!("Part 2: {},{}", res.0, res.1);
}

