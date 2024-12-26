use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Write;
use crate::Day;

pub struct Day24 {}

impl Day for Day24 {
    fn execute(&self) {
        println!("Day24");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day24/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day24/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let mut lines = contents.lines();

    let mut state = HashMap::<&str, bool>::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let split = line.split(": ").collect::<Vec<&str>>();
        state.insert(split[0], split[1] == "1");
    }

    let mut wire_to_outputs = HashMap::<&str, Vec<&str>>::new();
    let mut output_to_wires = HashMap::<&str, (&str, &str, &str)>::new();
    let mut output_to_count = HashMap::<&str, i32>::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let split = line.split(" ").collect::<Vec<&str>>();
        match wire_to_outputs.get_mut(split[0]) {
            Some(outputs) => { outputs.push(split[4]); },
            None => { wire_to_outputs.insert(split[0], vec![split[4]]); },
        }
        match wire_to_outputs.get_mut(split[2]) {
            Some(wires) => { wires.push(split[4]); },
            None => { wire_to_outputs.insert(split[2], vec![split[4]]); },
        }
        output_to_wires.insert(split[4], (split[0], split[2], split[1]));
        output_to_count.insert(split[4], 2);
    }

    let mut q = VecDeque::<&str>::new();
    for (k, _) in &state {
        let outputs = wire_to_outputs.get(k).unwrap();
        for output in outputs {
            let count = output_to_count.get_mut(output).unwrap();
            *count -= 1;
            if *count == 0 {
                q.push_back(output);
            }
        }
    }

    while !q.is_empty() {
        let wire = q.pop_front().unwrap();

        let (input1, input2, op) = output_to_wires.get(wire).unwrap();
        let a = state.get(input1).unwrap();
        let b = state.get(input2).unwrap();
        match *op {
            "AND" => { state.insert(wire, a & b); },
            "OR" => { state.insert(wire, a | b); },
            "XOR" => { state.insert(wire, a ^ b); },
            _ => { panic!("Unknown op: {}", op); },
        }

        let default = Vec::new();
        let outputs = wire_to_outputs.get(wire).unwrap_or(&default);
        for output in outputs {
            let count = output_to_count.get_mut(output).unwrap();
            *count -= 1;
            if *count == 0 {
                q.push_back(output);
            }
        }
    }

    let mut state_vector = state
        .iter()
        .filter(|(k, _)| k.get(0..1).unwrap() == "z")
        .collect::<Vec<(&&str, &bool)>>();
    state_vector.sort();

    let mut total = 0;
    for (i, (_, v)) in state_vector.iter().enumerate() {
        if **v {
            total += 2i64.pow(i as u32);
        }
    }

    println!("Part 1: {}", total);
}

fn part2(contents: String) {
    let mut lines = contents.lines();

    let mut state = HashMap::<String, bool>::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let split = line.split(": ").collect::<Vec<&str>>();
        state.insert(split[0].to_string(), false);
    }

    let mappings = HashMap::<String, String>::from_iter(vec![
        ("z07".to_string(), "vmv".to_string()),
        ("vmv".to_string(), "z07".to_string()),

        ("z20".to_string(), "kfm".to_string()),
        ("kfm".to_string(), "z20".to_string()),

        ("z28".to_string(), "hnv".to_string()),
        ("hnv".to_string(), "z28".to_string()),

        ("tqr".to_string(), "hth".to_string()),
        ("hth".to_string(), "tqr".to_string()),
    ]);

    let mut wire_to_outputs = HashMap::<String, Vec<String>>::new();
    let mut output_to_wires = HashMap::<String, (String, String, String)>::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }
        let split = line.split(" ").collect::<Vec<&str>>();

        let output = split[4].to_string();
        let input1 = split[0].to_string();
        let input2 = split[2].to_string();
        let op = split[1].to_string();
        let mapped_output = mappings.get(&output).unwrap_or(&output).to_string();

        match wire_to_outputs.get_mut(&input1) {
            Some(outputs) => { outputs.push(mapped_output.clone()); },
            None => { wire_to_outputs.insert(input1.clone(), vec![mapped_output.clone()]); },
        }
        match wire_to_outputs.get_mut(&input2) {
            Some(outputs) => { outputs.push(mapped_output.clone()); },
            None => { wire_to_outputs.insert(input2.clone(), vec![mapped_output.clone()]); },
        }
        output_to_wires.insert(mapped_output.clone(), (input1.clone(), input2.clone(), op.clone()));
    }

    write_graphviz(&output_to_wires);

    for i in 0..45 {
        let x_key = format!("x{number:0>2}", number=i);
        let y_key = format!("y{number:0>2}", number=i);

        *state.get_mut(&x_key).unwrap() = true;
        *state.get_mut(&y_key).unwrap() = true;

        get_wrong_bits(&state, &wire_to_outputs, &output_to_wires, i+1);

        *state.get_mut(&x_key).unwrap() = false;
        *state.get_mut(&y_key).unwrap() = false;
    }

    // write the graphviz file
    // print the incorrect results from adding each bit
    // find the connections in the graph that are incorrect

    println!("Part 2: {}", "hnv,hth,kfm,tqr,vmv,z07,z20,z28");
}

fn get_wrong_bits(
    state_original: &HashMap<String, bool>,
    wire_to_outputs: &HashMap<String, Vec<String>>,
    output_to_wires: &HashMap<String, (String, String, String)>,
    i: i32,
) {
    let mut state = state_original.clone();

    let mut output_to_count = HashMap::<String, i32>::new();
    for (output, _) in output_to_wires {
        output_to_count.insert(output.clone(), 2);
    }

    let mut q = VecDeque::<String>::new();
    for (k, _) in &state {
        let outputs = wire_to_outputs.get(k).unwrap();
        for output in outputs {
            let count = output_to_count.get_mut(output).unwrap();
            *count -= 1;
            if *count == 0 {
                q.push_back(output.clone());
            }
        }
    }

    while !q.is_empty() {
        let wire = q.pop_front().unwrap();

        let (input1, input2, op) = output_to_wires.get(&wire).unwrap();
        let a = state.get(input1).unwrap();
        let b = state.get(input2).unwrap();
        match op.as_str() {
            "AND" => { state.insert(wire.clone(), a & b); },
            "OR" => { state.insert(wire.clone(), a | b); },
            "XOR" => { state.insert(wire.clone(), a ^ b); },
            _ => { panic!("Unknown op: {}", op); },
        }

        let default = Vec::new();
        let outputs = wire_to_outputs.get(&wire).unwrap_or(&default);
        for output in outputs {
            let count = output_to_count.get_mut(output).unwrap();
            *count -= 1;
            if *count == 0 {
                q.push_back(output.clone());
            }
        }
    }

    let mut all_set = Vec::<&str>::new();
    for (k, v) in state.iter() {
        if *v {
            all_set.push(k);
        }
    }
    all_set.sort();

    let mut state_vector = state
        .iter()
        .filter(|(k, _)| k.get(0..1).unwrap() == "z")
        .collect::<Vec<(&String, &bool)>>();
    state_vector.sort();

    let mut total = 0;
    let mut num_set = false;
    let mut other_set = Vec::<i32>::new();
    for (i2, (_, v)) in state_vector.iter().enumerate() {
        if **v {
            if i2 == i as usize {
                num_set = true;
            } else {
                other_set.push(i2 as i32);
            }
            total += 2i64.pow(i2 as u32);
        }
    }
    println!("num: {}, {}_set: {}, other_set: {:?}, all_set: {:?}", total, i, num_set, other_set, all_set);
}

fn write_graphviz(output_to_wires: &HashMap<String, (String, String, String)>) {
    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("src/day24/output2")
        .unwrap();

    f.write(b"digraph G {\n").unwrap();
    for (output, (input1, input2, op)) in output_to_wires {
        f.write(format!("{} -> {} [label=\"{}\"]\n", input1, output, op).as_bytes()).unwrap();
        f.write(format!("{} -> {} [label=\"{}\"]\n", input2, output, op).as_bytes()).unwrap();
    }
    f.write(b"}\n").unwrap();
}

