use std::fs;
use std::env;
use crate::Day;

pub struct Day13 {}

impl Day for Day13 {
    fn execute(&self) {
        println!("Day13");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day13/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day13/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let mut total = 0;

    while line.is_some() {
        let a_line = line.unwrap();
        let a_line_split = a_line.split(" ").collect::<Vec<&str>>();
        let a_x = a_line_split[2].strip_prefix("X+").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap();
        let a_y = a_line_split[3].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        line = lines.next();
        let b_line = line.unwrap();
        let b_line_split = b_line.split(" ").collect::<Vec<&str>>();
        let b_x = b_line_split[2].strip_prefix("X+").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap();
        let b_y = b_line_split[3].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        line = lines.next();
        let p_line = line.unwrap();
        let p_line_split = p_line.split(" ").collect::<Vec<&str>>();
        let p_x = p_line_split[1].strip_prefix("X=").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap();
        let p_y = p_line_split[2].strip_prefix("Y=").unwrap().parse::<i64>().unwrap();

        let (a_presses, b_presses) = get_cheapest(a_x, a_y, b_x, b_y, p_x, p_y);
        if a_presses > 0 && b_presses > 0 {
            total += 3 * a_presses + b_presses;
        }

        lines.next();
        line = lines.next();
    }

    println!("Part 1: {}", total);
}

fn get_cheapest(a_x: i64, a_y: i64, b_x: i64, b_y: i64, p_x: i64, p_y: i64) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut a_presses = 0;
    let mut b_presses = 0;

    while x < p_x && y < p_y {
        x += b_x;
        y += b_y;
        b_presses += 1;
    }

    while b_presses >= 0 {
        if x == p_x && y == p_y {
            return (a_presses, b_presses);
        }

        while x > p_x || y > p_y {
            x -= b_x;
            y -= b_y;
            b_presses -= 1;
        }

        while x < p_x || y < p_y {
            x += a_x;
            y += a_y;
            a_presses += 1;
        }
    }

    return (-1, -1);
}

fn part2(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let mut total = 0;

    while line.is_some() {
        let a_line = line.unwrap();
        let a_line_split = a_line.split(" ").collect::<Vec<&str>>();
        let a_x = a_line_split[2].strip_prefix("X+").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap();
        let a_y = a_line_split[3].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        line = lines.next();
        let b_line = line.unwrap();
        let b_line_split = b_line.split(" ").collect::<Vec<&str>>();
        let b_x = b_line_split[2].strip_prefix("X+").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap();
        let b_y = b_line_split[3].strip_prefix("Y+").unwrap().parse::<i64>().unwrap();

        line = lines.next();
        let p_line = line.unwrap();
        let p_line_split = p_line.split(" ").collect::<Vec<&str>>();
        let p_x = p_line_split[1].strip_prefix("X=").unwrap().strip_suffix(",").unwrap().parse::<i64>().unwrap() + 10000000000000;
        let p_y = p_line_split[2].strip_prefix("Y=").unwrap().parse::<i64>().unwrap() + 10000000000000;

        let (a_presses, b_presses) = get_cheapest2(a_x, a_y, b_x, b_y, p_x, p_y);
        if a_presses > 0 && b_presses > 0 {
            total += 3 * a_presses + b_presses;
        }

        lines.next();
        line = lines.next();
    }

    println!("Part 2: {}", total);
}

fn get_cheapest2(a_x: i64, a_y: i64, b_x: i64, b_y: i64, p_x: i64, p_y: i64) -> (i64, i64) {
    // a_x * a_presses + b_x * b_presses = p_x
    // a_y * a_presses + b_y * b_presses = p_y
    // (p_x - b_x * b_presses) / a_x = a_presses
    // b_presses = (p_y * a_x - p_x * a_y) / (a_x * b_y - a_y * b_x)

    let b_presses = (p_y * a_x - p_x * a_y) / (a_x * b_y - a_y * b_x);
    let a_presses = (p_x - b_x * b_presses) / a_x;

    let x = a_x * a_presses + b_x * b_presses;
    let y = a_y * a_presses + b_y * b_presses;

    if x == p_x && y == p_y {
        return (a_presses, b_presses);
    }

    return (-1, -1);
}

