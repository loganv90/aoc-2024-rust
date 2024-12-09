use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::Day;

pub struct Day08 {}

impl Day for Day08 {
    fn execute(&self) {
        println!("Day08");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day08/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day08/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();
    let maxr = contents.lines().count() as i64;
    let maxc = contents.lines().next().unwrap().chars().count() as i64;

    let mut locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (r, row) in lines.enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            let locs = locations.entry(ch).or_insert(Vec::new());
            locs.push((r as i64, c as i64));
        }
    }

    let mut unique_locations: HashSet<(i64, i64)> = HashSet::new();

    for (_, locs) in locations.iter() {
        let antinodes = get_antinodes(&locs);
        unique_locations.extend(antinodes);
    }

    let res = unique_locations
        .iter()
        .filter(|(r, c)| { *r >= 0 && *c >= 0 && *r < maxr && *c < maxc })
        .count();

    println!("Part 1: {}", res);
}

fn get_antinodes(locs: &Vec<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut res = HashSet::new();

    for i in 0..locs.len() {
        for j in i+1..locs.len() {
            let (r1, c1) = locs[i];
            let (r2, c2) = locs[j];
            let (dr, dc) = (r2 - r1, c2 - c1);
            res.insert((r1 - dr, c1 - dc));
            res.insert((r2 + dr, c2 + dc));
        }
    }

    return res;
}

fn part2(contents: String) {
    let lines = contents.lines();
    let maxr = contents.lines().count() as i64;
    let maxc = contents.lines().next().unwrap().chars().count() as i64;

    let mut locations: HashMap<char, Vec<(i64, i64)>> = HashMap::new();

    for (r, row) in lines.enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            let locs = locations.entry(ch).or_insert(Vec::new());
            locs.push((r as i64, c as i64));
        }
    }

    let mut unique_locations: HashSet<(i64, i64)> = HashSet::new();

    for (_, locs) in locations.iter() {
        let antinodes = get_antinodes_repeat(&locs, maxr, maxc);
        unique_locations.extend(antinodes);
    }

    let res = unique_locations.iter().count();

    println!("Part 2: {}", res);
}

fn get_antinodes_repeat(locs: &Vec<(i64, i64)>, maxr: i64, maxc: i64) -> HashSet<(i64, i64)> {
    let mut res = HashSet::new();

    for i in 0..locs.len() {
        for j in i+1..locs.len() {
            let (mut r1, mut c1) = locs[i];
            let (mut r2, mut c2) = locs[j];
            let (dr, dc) = (r2 - r1, c2 - c1);

            while r1 >= 0 && c1 >= 0 && r1 < maxr && c1 < maxc {
                res.insert((r1, c1));
                r1 -= dr;
                c1 -= dc;
            }

            while r2 >= 0 && c2 >= 0 && r2 < maxr && c2 < maxc {
                res.insert((r2, c2));
                r2 += dr;
                c2 += dc;
            }
        }
    }

    return res;
}

