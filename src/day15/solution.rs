use std::fs;
use std::env;
use std::collections::VecDeque;
use std::collections::HashSet;
use crate::Day;

pub struct Day15 {}

impl Day for Day15 {
    fn execute(&self) {
        println!("Day15");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day15/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day15/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while !line.unwrap().is_empty() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            if ch == '@' {
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

    line = lines.next();
    while line.is_some() {
        for ch in line.unwrap().chars() {
            if ch == '<' {
                (r, c) = move_robot(&mut grid, r, c, 0, -1);
            } else if ch == '>' {
                (r, c) = move_robot(&mut grid, r, c, 0, 1);
            } else if ch == '^' {
                (r, c) = move_robot(&mut grid, r, c, -1, 0);
            } else if ch == 'v' {
                (r, c) = move_robot(&mut grid, r, c, 1, 0);
            }
        }
        line = lines.next();
    }

    println!("Part 1: {}", count_grid(&grid));
}

fn move_robot(grid: &mut Vec<Vec<char>>, r: i32, c: i32, dr: i32, dc: i32) -> (i32, i32) {
    let nr = r + dr;
    let nc = c + dc;
    let ch = grid[nr as usize][nc as usize];
    if ch == '#' {
        return (r, c);
    } else if ch == '.' {
        return (nr, nc);
    } else if ch != 'O' {
        panic!("Unexpected character: {}", ch);
    }

    let mut nnr = nr + dr;
    let mut nnc = nc + dc;
    let mut nch = grid[nnr as usize][nnc as usize];
    while nch == 'O' {
        nnr += dr;
        nnc += dc;
        nch = grid[nnr as usize][nnc as usize];
    }

    if nch == '#' {
        return (r, c);
    } else if nch != '.' {
        panic!("Unexpected character: {}", nch);
    }

    grid[nr as usize][nc as usize] = '.';
    grid[nnr as usize][nnc as usize] = 'O';

    return (nr, nc);
}

fn count_grid(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == 'O' {
                count += 100 * r as i32 + c as i32;
            }
        }
    }
    return count;
}

fn part2(contents: String) {
    let mut lines = contents.lines();
    let mut line = lines.next();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while !line.unwrap().is_empty() {
        let mut row: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            if ch == '@' {
                r = grid.len() as i32;
                c = row.len() as i32;
                row.push('.');
                row.push('.');
            } else if ch == 'O' {
                row.push('[');
                row.push(']');
            } else {
                row.push(ch);
                row.push(ch);
            }
        }
        grid.push(row);
        line = lines.next();
    }

    line = lines.next();
    while line.is_some() {
        for ch in line.unwrap().chars() {
            if ch == '<' {
                (r, c) = move_horizontal(&mut grid, r, c, -1);
            } else if ch == '>' {
                (r, c) = move_horizontal(&mut grid, r, c, 1);
            } else if ch == '^' {
                (r, c) = move_vertical(&mut grid, r, c, -1);
            } else if ch == 'v' {
                (r, c) = move_vertical(&mut grid, r, c, 1);
            }
        }

        line = lines.next();
    }

    println!("Part 2: {}", count_grid2(&grid));
}

fn count_grid2(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if *ch == '[' {
                count += 100 * r as i32 + c as i32;
            }
        }
    }
    return count;
}

fn move_vertical(grid: &mut Vec<Vec<char>>, r: i32, c: i32, dr: i32) -> (i32, i32) {
    let mut nr = r + dr;
    let mut nc = c;
    let mut ch = grid[nr as usize][nc as usize];
    if ch == '#' {
        return (r, c);
    } else if ch == '.' {
        return (nr, nc);
    } else if ch != '[' && ch != ']' {
        panic!("Unexpected character: {}", ch);
    }

    let mut q = VecDeque::new();
    let mut v = HashSet::<(i32, i32)>::new();
    q.push_back((r, c));

    let mut wall = false;
    while q.len() > 0 {
        (nr, nc) = q.pop_front().unwrap();

        ch = grid[(nr + dr) as usize][nc as usize];
        if ch == '.' {
            continue;
        } else if ch == '#' {
            wall = true;
            break;
        } else if ch != '[' && ch != ']' {
            panic!("Unexpected character: {}", ch);
        }

        v.insert((nr + dr, nc));
        q.push_back((nr + dr, nc));
        if ch == '[' {
            v.insert((nr + dr, nc + 1));
            q.push_back((nr + dr, nc + 1));
        } else if ch == ']' {
            v.insert((nr + dr, nc - 1));
            q.push_back((nr + dr, nc - 1));
        } else {
            panic!("Unexpected character: {}", ch);
        }
    }
    if wall {
        return (r, c);
    }

    let mut sorted = v.iter().collect::<Vec<&(i32, i32)>>();
    sorted.sort_by(|a, b| {
        if dr < 0 {
            return a.0.cmp(&b.0);
        } else {
            return b.0.cmp(&a.0);
        }
    });

    for &(nnr, nnc) in sorted {
        grid[(nnr + dr) as usize][nnc as usize] = grid[nnr as usize][nnc as usize];
        grid[nnr as usize][nnc as usize] = '.';
    }

    return (r + dr, c);
}

fn move_horizontal(grid: &mut Vec<Vec<char>>, r: i32, c: i32, dc: i32) -> (i32, i32) {
    let nr = r;
    let mut nc = c + dc;
    let mut ch = grid[nr as usize][nc as usize];
    if ch == '#' {
        return (r, c);
    } else if ch == '.' {
        return (nr, nc);
    } else if ch != '[' && ch != ']' {
        panic!("Unexpected character: {}", ch);
    }

    let mut v = Vec::<(i32, i32)>::new();
    while ch == '[' || ch == ']' {
        v.push((nr, nc));
        v.push((nr, nc + dc));

        nc += dc * 2;
        ch = grid[nr as usize][nc as usize];
    }
    if ch == '#' {
        return (r, c);
    }

    v.reverse();
    for (nnr, nnc) in v {
        grid[nnr as usize][(nnc + dc) as usize] = grid[nnr as usize][nnc as usize];
        grid[nnr as usize][nnc as usize] = '.';
    }

    return (r, c + dc);
}

