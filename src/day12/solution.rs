use std::fs;
use std::env;
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::Day;

pub struct Day12 {}

impl Day for Day12 {
    fn execute(&self) {
        println!("Day12");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day12/test1");
        let mut test2_path = base_path.clone();
        test2_path.push("src/day12/test2");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day12/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&test2_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        let mut row_v: Vec<bool> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
            row_v.push(false);
        }
        grid.push(row);
        visited.push(row_v);
    }

    let mut total = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if visited[r][c] {
                continue;
            }

            let (area, perimeter) = get_area_and_perimeter(&grid, &mut visited, ch, r as i32, c as i32);
            total += area * perimeter;
        }
    }

    println!("Part 1: {}", total);
}

fn get_area_and_perimeter(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, ch: &char, r: i32, c: i32) -> (i32, i32) {
    let mut area = 0;
    let mut perimeter = 0;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut q = VecDeque::new();
    q.push_back((r, c));

    while q.len() > 0 {
        let (r, c) = q.pop_front().unwrap();
        if visited[r as usize][c as usize] {
            continue;
        }

        for (dr, dc) in directions.iter() {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[nr as usize].len() as i32 {
                perimeter += 1;
                continue;
            } else if grid[nr as usize][nc as usize] != *ch {
                perimeter += 1;
                continue;
            }

            q.push_back((nr, nc));
        }

        visited[r as usize][c as usize] = true;
        area += 1;
    }

    return (area, perimeter);
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();

    for line in lines {
        let mut row: Vec<char> = Vec::new();
        let mut row_v: Vec<bool> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
            row_v.push(false);
        }
        grid.push(row);
        visited.push(row_v);
    }

    let mut total = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if visited[r][c] {
                continue;
            }

            let (area, sides) = get_area_and_sides(&grid, &mut visited, ch, r as i32, c as i32);
            total += area * sides;
        }
    }

    println!("Part 2: {}", total);
}

fn get_area_and_sides(grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, ch: &char, r: i32, c: i32) -> (i32, i32) {
    let mut area = 0;
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut corners = HashMap::<(i32, i32), i32>::new();

    let mut q = VecDeque::new();
    q.push_back((r, c));

    let mut increment_corner = |r: i32, c: i32| {
        match corners.get_mut(&(r, c)) {
            Some(cur) => { *cur += 1; },
            None => { corners.insert((r, c), 1); },
        }
    };

    let count_double_corner = |r1: i32, c1: i32, r2: i32, c2: i32| -> bool {
        return
            r1 >= 0 && r1 < grid.len() as i32 && c1 >= 0 && c1 < grid[r1 as usize].len() as i32 &&
            r2 >= 0 && r2 < grid.len() as i32 && c2 >= 0 && c2 < grid[r2 as usize].len() as i32 &&
            grid[r1 as usize][c1 as usize] == *ch && grid[r2 as usize][c2 as usize] == *ch;
    };

    while q.len() > 0 {
        let (r, c) = q.pop_front().unwrap();
        if visited[r as usize][c as usize] {
            continue;
        }

        increment_corner(r, c);
        increment_corner(r + 1, c);
        increment_corner(r, c + 1);
        increment_corner(r + 1, c + 1);

        for (dr, dc) in directions.iter() {
            let nr = r + dr;
            let nc = c + dc;

            if nr < 0 || nr >= grid.len() as i32 || nc < 0 || nc >= grid[nr as usize].len() as i32 {
                continue;
            } else if grid[nr as usize][nc as usize] != *ch {
                continue;
            }

            q.push_back((nr, nc));
        }

        visited[r as usize][c as usize] = true;
        area += 1;
    }

    let mut corner_count = corners.iter().filter(|(_, &v)| v == 1 || v == 3).count() as i32;
    corner_count += 2 * corners.iter().filter(|(&(r, c), &v)| { v == 2 && (count_double_corner(r, c, r-1, c-1) || count_double_corner(r, c-1, r-1, c)) }).count() as i32;

    return (area, corner_count);
}

