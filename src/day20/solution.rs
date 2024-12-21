use std::fs;
use std::env;
use std::collections::HashMap;
use crate::Day;

pub struct Day20 {}

impl Day for Day20 {
    fn execute(&self) {
        println!("Day20");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day20/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day20/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut rs = -1;
    let mut cs = -1;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for (r, line) in lines.enumerate() {
        let mut row = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                rs = r as i32;
                cs = c as i32;
                row.push(0);
            } else if ch == '#' {
                row.push(-1);
            } else {
                row.push(0);
            }
        }
        grid.push(row);
    }

    assign_distances(&mut grid, rs, cs);
    let cheats = collect_cheats(&grid, 2);

    let mut total = 0;
    for (k, v) in cheats.iter() {
        if *k < 100 {
            continue;
        }
        total += v;
    }

    println!("Part 1: {}", total);
}

fn assign_distances(grid: &mut Vec<Vec<i32>>, r: i32, c: i32) {
    let mut row_current = r;
    let mut col_current = c;
    let mut dis_current = 0;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    loop {
        grid[row_current as usize][col_current as usize] = dis_current;
        let mut row_next = -1;
        let mut col_next = -1;

        for direction in directions.iter() {
            let row_new = row_current + direction.0;
            let col_new = col_current + direction.1;
            if row_new < 0 || row_new >= grid.len() as i32 || col_new < 0 || col_new >= grid[row_new as usize].len() as i32 {
                continue;
            }
            
            let dis_new = grid[row_new as usize][col_new as usize];
            if dis_new != 0 {
                continue;
            }

            row_next = row_new;
            col_next = col_new;
        }
        if row_next == -1 || col_next == -1 {
            break;
        }

        dis_current += 1;
        row_current = row_next;
        col_current = col_next;
    }
}

fn collect_cheats(grid: &Vec<Vec<i32>>, cheat_distance: i32) -> HashMap<i32, i32> {
    let mut cheats: HashMap<i32, i32> = HashMap::new();

    let mut positions = Vec::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell >= 0 {
                positions.push((r as i32, c as i32, *cell));
            }
        }
    }

    for i in 0..positions.len() {
        for j in i+1..positions.len() {
            let (r1, c1, d1) = positions[i];
            let (r2, c2, d2) = positions[j];

            let diff = (r1 - r2).abs() + (c1 - c2).abs();
            if diff > cheat_distance {
                continue;
            }

            let skip = (d1 - d2).abs() - diff;
            match cheats.get_mut(&skip) {
                Some(cur) => { *cur += 1; },
                None => { cheats.insert(skip, 1); },
            }
        }
    }

    return cheats
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut rs = -1;
    let mut cs = -1;
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for (r, line) in lines.enumerate() {
        let mut row = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                rs = r as i32;
                cs = c as i32;
                row.push(0);
            } else if ch == '#' {
                row.push(-1);
            } else {
                row.push(0);
            }
        }
        grid.push(row);
    }

    assign_distances(&mut grid, rs, cs);
    let cheats = collect_cheats(&grid, 20);

    let mut total = 0;
    for (k, v) in cheats.iter() {
        if *k < 100 {
            continue;
        }
        total += v;
    }

    println!("Part 2: {}", total);
}

