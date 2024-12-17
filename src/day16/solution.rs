use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::Day;

pub struct Day16 {}

impl Day for Day16 {
    fn execute(&self) {
        println!("Day16");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day16/test1");
        let mut test2_path = base_path.clone();
        test2_path.push("src/day16/test2");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day16/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&test2_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&test2_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let di = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while line.is_some() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            if ch == 'S' {
                r = grid.len() as i32;
                c = row.len() as i32;
                row.push('.');
            } else {
                row.push(ch);
            }
        }
        grid.push(row);
        line = lines.next();
    }

    for (k, v) in &visited {
        println!("{:?} -> {}", k, v);
    }

    println!("Part 1: {}", find_lowest(&grid, &mut visited, &directions, 0, r, c, di));
}

fn find_lowest(
    grid: &Vec<Vec<char>>,
    v: &mut HashMap<(i32, i32, i32), i32>,
    d: &Vec<(i32, i32)>,
    s: i32,
    r: i32,
    c: i32,
    di: i32
) -> i32 {
    let existing_s = v.get(&(r, c, di));
    if existing_s.is_some() && *existing_s.unwrap() < s {
        return -1;
    }

    let ch = grid[r as usize][c as usize];
    if ch == '#' {
        return -1;
    } else if ch == 'E' {
        return s;
    } else if ch != '.' {
        panic!("Invalid character: {}", ch);
    }

    v.insert((r, c, di), s);
    let mut lowest = 100000000;

    let mut dir = d[di as usize];
    let mut candidate = find_lowest(grid, v, d, s + 1, r + dir.0, c + dir.1, di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    let next_di = (di + 1) % 4;
    dir = d[next_di as usize];
    candidate = find_lowest(grid, v, d, s + 1001, r + dir.0, c + dir.1, next_di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    let prev_di = (di + 3) % 4;
    dir = d[prev_di as usize];
    candidate = find_lowest(grid, v, d, s + 1001, r + dir.0, c + dir.1, prev_di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    return lowest;
}

fn part2(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let directions: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut visited: HashMap<(i32, i32, i32), i32> = HashMap::new();
    let di = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while line.is_some() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            if ch == 'S' {
                r = grid.len() as i32;
                c = row.len() as i32;
                row.push('.');
            } else {
                row.push(ch);
            }
        }
        grid.push(row);
        line = lines.next();
    }

    let mut l: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let lowest = find_lowest2(&grid, &mut visited, &directions, &mut l, 0, r, c, di);

    println!("Part 2: {}", l[&lowest].len());
}

fn find_lowest2(
    grid: &Vec<Vec<char>>,
    v: &mut HashMap<(i32, i32, i32), i32>,
    d: &Vec<(i32, i32)>,
    l: &mut HashMap<i32, HashSet<(i32, i32)>>,
    s: i32,
    r: i32,
    c: i32,
    di: i32
) -> i32 {
    let existing_s = v.get(&(r, c, di));
    if existing_s.is_some() && *existing_s.unwrap() < s {
        return -1;
    }

    let ch = grid[r as usize][c as usize];
    if ch == '#' {
        return -1;
    } else if ch == 'E' {
        match l.get_mut(&s) {
            Some(set) => {
                set.insert((r, c));
            },
            None => {
                let mut set = HashSet::new();
                set.insert((r, c));
                l.insert(s, set);
            },
        }
        return s;
    } else if ch != '.' {
        panic!("Invalid character: {}", ch);
    }

    v.insert((r, c, di), s);
    let mut lowest = 100000000;

    let mut dir = d[di as usize];
    let mut candidate = find_lowest2(grid, v, d, l, s + 1, r + dir.0, c + dir.1, di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    let next_di = (di + 1) % 4;
    dir = d[next_di as usize];
    candidate = find_lowest2(grid, v, d, l, s + 1001, r + dir.0, c + dir.1, next_di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    let prev_di = (di + 3) % 4;
    dir = d[prev_di as usize];
    candidate = find_lowest2(grid, v, d, l, s + 1001, r + dir.0, c + dir.1, prev_di);
    if candidate >= 0 {
        lowest = std::cmp::min(lowest, candidate);
    }

    match l.get_mut(&lowest) {
        Some(set) => {
            set.insert((r, c));
        },
        None => {
            let mut set = HashSet::new();
            set.insert((r, c));
            l.insert(lowest, set);
        },
    }

    return lowest;
}

