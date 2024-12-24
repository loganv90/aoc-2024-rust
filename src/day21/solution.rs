use std::fs;
use std::env;
use std::collections::HashMap;
use crate::Day;

pub struct Day21 {}

impl Day for Day21 {
    fn execute(&self) {
        println!("Day21");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day21/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day21/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut codes = Vec::new();
    for line in lines {
        codes.push(line);
    }

    let mut total = 0;
    for code in codes {
        let min = numberpad_recurse(code.to_string(), 2);
        let num = code[0..3].parse::<i32>().unwrap();
        total += num * min;
    }

    println!("Part 1: {}", total);
}

fn numberpad_recurse(code: String, remaining_steps: i32) -> i32 {
    let numberpad_chars = HashMap::<char, (i32, i32)>::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    let numberpad = Vec::<Vec<bool>>::from([
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([false, true, true]),
    ]);
    let mut numberpad_current_row = 3;
    let mut numberpad_current_col = 2;
    let mut total = 0;

    for ch in code.chars() {
        let (row, col) = numberpad_chars.get(&ch).unwrap();
        let valid_paths = get_valid_paths(&numberpad, numberpad_current_row, numberpad_current_col, *row, *col);
        let mut min = i32::MAX;

        for path in valid_paths {
            let next_code = get_code_from_path(path);

            if remaining_steps > 0 {
                min = min.min(arrowkeys_recurse(next_code, remaining_steps - 1));
            } else {
                min = min.min(next_code.len() as i32);
            }
        }

        total += min;
        numberpad_current_row = *row;
        numberpad_current_col = *col;
    }

    return total;
}

fn arrowkeys_recurse(code: String, remaining_steps: i32) -> i32 {
    let arrowkeys_chars = HashMap::<char, (i32, i32)>::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
    let arrowkeys = Vec::<Vec<bool>>::from([
        Vec::<bool>::from([false, true, true]),
        Vec::<bool>::from([true, true, true]),
    ]);
    let mut arrowkeys_current_row = 0;
    let mut arrowkeys_current_col = 2;
    let mut total = 0;

    for ch in code.chars() {
        let (row, col) = arrowkeys_chars.get(&ch).unwrap();
        let valid_paths = get_valid_paths(&arrowkeys, arrowkeys_current_row, arrowkeys_current_col, *row, *col);
        let mut min = i32::MAX;

        for path in valid_paths {
            let next_code = get_code_from_path(path);

            if remaining_steps > 0 {
                min = min.min(arrowkeys_recurse(next_code, remaining_steps - 1));
            } else {
                min = min.min(next_code.len() as i32);
            }
        }

        total += min;
        arrowkeys_current_row = *row;
        arrowkeys_current_col = *col;
    }

    return total;
}

fn get_valid_paths(
    grid: &Vec<Vec<bool>>,
    row_start: i32,
    col_start: i32,
    row_end: i32,
    col_end: i32,
) -> Vec<((i32, i32), (i32, i32))> {
    let dr = row_end - row_start;
    let dc = col_end - col_start;
    if dr == 0 || dc == 0 {
        return Vec::from([((dr, dc), (0, 0))]);
    }

    let mut valid_paths = Vec::new();
    let corner1_exists = grid[row_start as usize][col_end as usize];
    if corner1_exists {
        valid_paths.push(((0, dc), (dr, 0)));
    }
    let corner2_exists = grid[row_end as usize][col_start as usize];
    if corner2_exists {
        valid_paths.push(((dr, 0), (0, dc)));
    }
    return valid_paths
}

fn get_code_from_path(
    path: ((i32, i32), (i32, i32)),
) -> String {
    let mut code = String::new();

    if path.0.0 > 0 {
        code += &"v".repeat(path.0.0 as usize);
    } else if path.0.0 < 0 {
        code += &"^".repeat(-path.0.0 as usize);
    }
    if path.0.1 > 0 {
        code += &">".repeat(path.0.1 as usize);
    } else if path.0.1 < 0 {
        code += &"<".repeat(-path.0.1 as usize);
    }
    if path.1.0 > 0 {
        code += &"v".repeat(path.1.0 as usize);
    } else if path.1.0 < 0 {
        code += &"^".repeat(-path.1.0 as usize);
    }
    if path.1.1 > 0 {
        code += &">".repeat(path.1.1 as usize);
    } else if path.1.1 < 0 {
        code += &"<".repeat(-path.1.1 as usize);
    }

    code.push_str("A");
    return code
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut codes = Vec::new();
    for line in lines {
        codes.push(line);
    }

    let mut memo = HashMap::new();

    let mut total = 0;
    for code in codes {
        let min = numberpad_recurse2(code.to_string(), 25, &mut memo);
        let num = code[0..3].parse::<i64>().unwrap();
        total += num * min;
    }

    println!("Part 2: {}", total);
}

fn numberpad_recurse2(code: String, remaining_steps: i32, memo: &mut HashMap<(String, i32), i64>) -> i64 {
    let numberpad_chars = HashMap::<char, (i32, i32)>::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    let numberpad = Vec::<Vec<bool>>::from([
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([true, true, true]),
        Vec::<bool>::from([false, true, true]),
    ]);
    let mut numberpad_current_row = 3;
    let mut numberpad_current_col = 2;
    let mut total = 0;

    for ch in code.chars() {
        let (row, col) = numberpad_chars.get(&ch).unwrap();
        let valid_paths = get_valid_paths(&numberpad, numberpad_current_row, numberpad_current_col, *row, *col);
        let mut min = i64::MAX;

        for path in valid_paths {
            let next_code = get_code_from_path(path);

            if remaining_steps > 0 {
                min = min.min(arrowkeys_recurse2(next_code, remaining_steps - 1, memo));
            } else {
                min = min.min(next_code.len() as i64);
            }
        }

        total += min;
        numberpad_current_row = *row;
        numberpad_current_col = *col;
    }

    return total;
}

fn arrowkeys_recurse2(code: String, remaining_steps: i32, memo: &mut HashMap<(String, i32), i64>) -> i64 {
    if let Some(&val) = memo.get(&(code.clone(), remaining_steps)) {
        return val;
    }

    let arrowkeys_chars = HashMap::<char, (i32, i32)>::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
    let arrowkeys = Vec::<Vec<bool>>::from([
        Vec::<bool>::from([false, true, true]),
        Vec::<bool>::from([true, true, true]),
    ]);
    let mut arrowkeys_current_row = 0;
    let mut arrowkeys_current_col = 2;
    let mut total = 0;

    for ch in code.chars() {
        let (row, col) = arrowkeys_chars.get(&ch).unwrap();
        let valid_paths = get_valid_paths(&arrowkeys, arrowkeys_current_row, arrowkeys_current_col, *row, *col);
        let mut min = i64::MAX;

        for path in valid_paths {
            let next_code = get_code_from_path(path);

            if remaining_steps > 0 {
                min = min.min(arrowkeys_recurse2(next_code, remaining_steps - 1, memo));
            } else {
                min = min.min(next_code.len() as i64);
            }
        }

        total += min;
        arrowkeys_current_row = *row;
        arrowkeys_current_col = *col;
    }

    memo.insert((code.clone(), remaining_steps), total);
    return total;
}

