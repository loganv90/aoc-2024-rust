use std::fs;
use std::env;
use crate::Day;

pub struct Day17 {}

impl Day for Day17 {
    fn execute(&self) {
        println!("Day17");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day17/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day17/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let a_reg = lines.next();
    let b_reg = lines.next();
    let c_reg = lines.next();
    lines.next();
    let p_input = lines.next();

    let mut a = a_reg.unwrap().strip_prefix("Register A: ").unwrap().parse::<i32>().unwrap();
    let mut b = b_reg.unwrap().strip_prefix("Register B: ").unwrap().parse::<i32>().unwrap();
    let mut c = c_reg.unwrap().strip_prefix("Register C: ").unwrap().parse::<i32>().unwrap();
    let program = p_input.unwrap().strip_prefix("Program: ").unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut instruction: i32 = 0;
    let mut output = Vec::new();

    while instruction < program.len() as i32 {
        let opcode = program[instruction as usize];
        let operand = program[instruction as usize + 1];
        let combo = match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid operand"),
        };

        match opcode {
            0 => {
                a = a >> combo;
                instruction += 2;
            },
            1 => {
                b = b ^ operand;
                instruction += 2;
            },
            2 => {
                b = combo % 8;
                instruction += 2;
            },
            3 => {
                if a != 0 {
                    instruction = operand;
                } else {
                    instruction += 2;
                }
            },
            4 => {
                b = b ^ c;
                instruction += 2;
            },
            5 => {
                output.push(combo % 8);
                instruction += 2;
            },
            6 => {
                b = a >> combo;
                instruction += 2;
            },
            7 => {
                c = a >> combo;
                instruction += 2;
            },
            _ => panic!("Invalid opcode"),
        }
    }

    let res = output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    println!("Part 1: {}", res);
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    lines.next();
    lines.next();
    lines.next();
    lines.next();
    let p_input = lines.next();

    let program = p_input.unwrap().strip_prefix("Program: ").unwrap().split(",").map(|x| x.parse::<i128>().unwrap()).collect::<Vec<i128>>();

    let res = find_program(&program, program.len() as i128 - 1, 0);

    println!("Part 2: {}", res);
}

fn execute_iteration(mut a: i128) -> (i128, i128, i128, i128) {
    let mut b = a & 7;
    b = b ^ 3;
    let c = a >> b;
    b = b ^ c;
    b = b ^ 3;
    a = a >> 3;
    let out = b & 7;
    return (a, b, c, out);
}

fn run_program(mut a: i128) -> Vec<i128> {
    let mut output = Vec::new();
    loop {
        let (r_a, _, _, r_out) = execute_iteration(a);
        a = r_a;
        output.push(r_out);

        if a <= 0 {
            break;
        }
    }
    return output;
}

fn find_program(program: &Vec<i128>, index: i128, res: i128) -> i128 {
    if index < 0 {
        return res;
    }

    let mut r = -1;

    for i in 0..8 {
        let a = (res << 3) + i;
        let output = run_program(a);

        if output[0] == program[index as usize] {
            let next_res = find_program(program, index - 1, a);
            if res == 0 { // 0 on first iteration doesn't work, so return last result
                r = next_res;
            } else if next_res > -1{
                r = next_res;
                break;
            }
        }
    }

    return r;
}

