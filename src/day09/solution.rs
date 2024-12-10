use std::fs;
use std::env;
use crate::Day;

pub struct Day09 {}

impl Day for Day09 {
    fn execute(&self) {
        println!("Day09");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day09/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day09/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut disk = Vec::new();
    let line = lines.next().unwrap();
    for ch in line.chars() {
        disk.push(ch.to_digit(10).unwrap());
    }

    let mut total = 0;
    let mut left = 0;
    let mut right = disk.len() - 1;
    if right % 2 != 0 {
        right -= 1;
    }
    let mut id = 0;

    while left <= right {
        if left % 2 == 0 {
            // file
            let l = disk[left];
            if l > 0 {
                disk[left] -= 1;
                total += left / 2 * id;
                id += 1;
            } else {
                left += 1;
            }
        } else {
            // space
            let l = disk[left];
            if l > 0 {
                let r = disk[right];
                if r > 0 {
                    disk[right] -= 1;
                    disk[left] -= 1;
                    total += right / 2 * id;
                    id += 1;
                } else {
                    right -= 2;
                }
            } else {
                left += 1;
            }
        }
    }

    println!("Part 1: {}", total);
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let mut disk: Vec<(u64, u64)> = Vec::new();
    let line = lines.next().unwrap();
    for (i, ch) in line.chars().enumerate() {
        disk.push((ch.to_digit(10).unwrap() as u64, i as u64 / 2));
    }

    let mut right = disk.len() - 1;
    if right % 2 != 0 {
        right -= 1;
    }

    while right > 0 {
        let (required, id) = disk[right];
        match find_space(&disk, required, right) {
            Some(space) => {
                let (required, _) = disk.remove(right);
                disk.insert(right, (0, 0));
                disk.insert(right, (required, 0));
                disk.insert(right, (0, 0));

                let (available, _) = disk.remove(space);
                disk.insert(space, (available - required, 0));
                disk.insert(space, (required, id));
                disk.insert(space, (0, 0));
            },
            None => {
                right -= 2;
            },
        }
    }

    let res = count_disk(&disk);

    println!("Part 2: {}", res);
}

fn find_space(disk: &Vec<(u64, u64)>, space: u64, end: usize) -> Option<usize> {
    let mut start = 1;
    while start < end {
        let (size, _) = disk[start];

        if size >= space {
            return Some(start);
        }

        start += 2;
    }
    return None;
}

fn count_disk(disk: &Vec<(u64, u64)>) -> u64 {
    let mut total = 0;
    let mut index = 0;

    for (i, (size, id)) in disk.iter().enumerate() {
        if i % 2 == 0 {
            // file
            for _ in 0..*size {
                total += index * id;
                index += 1;
            }
        } else {
            // space
            index += *size;
        }
    }

    return total;
}

