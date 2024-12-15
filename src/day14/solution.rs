use std::fs;
use std::env;
use std::io::Write;
use crate::Day;

pub struct Day14 {}

impl Day for Day14 {
    fn execute(&self) {
        println!("Day14");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day14/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day14/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let first_line = lines.next().unwrap();
    let first_line_split: Vec<&str> = first_line.split(" ").collect();
    let width = first_line_split[0].strip_prefix("w=").unwrap().parse::<i32>().unwrap();
    let height = first_line_split[1].strip_prefix("h=").unwrap().parse::<i32>().unwrap();

    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut line = lines.next();
    while line.is_some() {
        let line_split: Vec<&str> = line.unwrap().split(" ").collect();
        let p_split: Vec<i32> = line_split[0].strip_prefix("p=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let v_split: Vec<i32> = line_split[1].strip_prefix("v=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        robots.push((p_split[0], p_split[1], v_split[0], v_split[1]));
        line = lines.next();
    }

    for _ in 0..100 {
        for robot in &mut robots {
            robot.0 += robot.2;
            while robot.0 >= width {
                robot.0 -= width;
            }
            while robot.0 < 0 {
                robot.0 += width;
            }

            robot.1 += robot.3;
            while robot.1 >= height {
                robot.1 -= height;
            }
            while robot.1 < 0 {
                robot.1 += height;
            }
        }
    }

    let mut nw = 0;
    let mut ne = 0;
    let mut sw = 0;
    let mut se = 0;
    for robot in &robots {
        if robot.0 < width / 2 && robot.1 < height / 2 {
            nw += 1;
        } else if robot.0 > width / 2 && robot.1 < height / 2 {
            ne += 1;
        } else if robot.0 < width / 2 && robot.1 > height / 2 {
            sw += 1;
        } else if robot.0 > width / 2 && robot.1 > height / 2 {
            se += 1;
        }
    }

    println!("Part 1: {}", nw * ne * sw * se);
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let first_line = lines.next().unwrap();
    let first_line_split: Vec<&str> = first_line.split(" ").collect();
    let width = first_line_split[0].strip_prefix("w=").unwrap().parse::<i32>().unwrap();
    let height = first_line_split[1].strip_prefix("h=").unwrap().parse::<i32>().unwrap();

    let mut robots: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut line = lines.next();
    while line.is_some() {
        let line_split: Vec<&str> = line.unwrap().split(" ").collect();
        let p_split: Vec<i32> = line_split[0].strip_prefix("p=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        let v_split: Vec<i32> = line_split[1].strip_prefix("v=").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        robots.push((p_split[0], p_split[1], v_split[0], v_split[1]));
        line = lines.next();
    }

    for i in 0..10000 {
        for robot in &mut robots {
            robot.0 += robot.2;
            while robot.0 >= width {
                robot.0 -= width;
            }
            while robot.0 < 0 {
                robot.0 += width;
            }

            robot.1 += robot.3;
            while robot.1 >= height {
                robot.1 -= height;
            }
            while robot.1 < 0 {
                robot.1 += height;
            }
        }

        if (i - 67) % 101 == 0 {
            write_robots(&robots, width, height, i);
        }
    }

    // seeing clusters in output at: 67, 168, 269, ...
    // limit output to instances of: (i - 67) % 101 == 0
    // scan output and find tree at: 7036
    // add 1 for answer: 7037

    println!("Part 2: {}", 7037);
}

fn write_robots(robots: &Vec<(i32, i32, i32, i32)>, width: i32, height: i32, i: i32) {
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/day14/output2")
        .unwrap();

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width as usize]; height as usize];
    for robot in robots {
        grid[robot.1 as usize][robot.0 as usize] = '#';
    }
    for row in grid {
        f.write(row.iter().collect::<String>().as_bytes()).unwrap();
        f.write(b"\n").unwrap();
    }
    f.write(format!("{}\n", i).as_bytes()).unwrap();
}

