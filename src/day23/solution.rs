use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::Day;

pub struct Day23 {}

impl Day for Day23 {
    fn execute(&self) {
        println!("Day23");

        let base_path = env::current_dir().unwrap();
        let mut test1_path = base_path.clone();
        test1_path.push("src/day23/test1");
        let mut input1_path = base_path.clone();
        input1_path.push("src/day23/input1");

        part1(fs::read_to_string(&test1_path).unwrap());
        part1(fs::read_to_string(&input1_path).unwrap());

        part2(fs::read_to_string(&test1_path).unwrap());
        part2(fs::read_to_string(&input1_path).unwrap());
    }
}

fn part1(contents: String) {
    let lines = contents.lines();

    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for line in lines {
        let num = line.split("-").collect::<Vec<&str>>();
        match graph.get_mut(num[0]) {
            Some(v) => {
                v.push(num[1]);
            },
            None => {
                graph.insert(num[0], vec![num[1]]);
            },
        }
        match graph.get_mut(num[1]) {
            Some(v) => {
                v.push(num[0]);
            },
            None => {
                graph.insert(num[1], vec![num[0]]);
            },
        }
    }

    let mut total = 0;
    for (k, v) in graph.iter() {
        for i in 0..v.len() {
            for j in i+1..v.len() {
                let neighbor1 = v[i];
                let neighbor2 = v[j];

                match graph.get(neighbor1) {
                    Some(n1) => {
                        if n1.contains(&neighbor2) && (neighbor1.get(0..1).unwrap() == "t" || neighbor2.get(0..1).unwrap() == "t" || k.get(0..1).unwrap() == "t") {
                            total += 1;
                        }
                    },
                    None => {},
                }
            }
        }
    }

    println!("Part 1: {}", total / 3);
}

struct TreeNode {
    children: HashMap<String, TreeNode>,
}

impl TreeNode {
    fn new() -> TreeNode {
        TreeNode {
            children: HashMap::new(),
        }
    }

    fn insert(&mut self, word: &str, graph: &HashMap<&str, HashSet<&str>>) {
        for (k, v) in self.children.iter_mut() {
            let neighbors = graph.get(k.as_str()).unwrap();
            if neighbors.contains(word) {
                v.insert(word, graph);
            }
        }
        self.children.insert(word.to_string(), TreeNode::new());
    }

    fn height(&self) -> (i32, Vec<String>) {
        let mut height = 0;
        let mut vector = Vec::<String>::new();

        for (k, v) in self.children.iter() {
            let (candidate_height, mut candidate_vector) = v.height();

            if candidate_height > height {
                candidate_vector.push(k.to_string());

                height = candidate_height;
                vector = candidate_vector;
            }
        }

        return (height + 1, vector);
    }
}

fn part2(contents: String) {
    let lines = contents.lines();

    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for line in lines {
        let num = line.split("-").collect::<Vec<&str>>();
        match graph.get_mut(num[0]) {
            Some(v) => {
                v.insert(num[1]);
            },
            None => {
                let mut set = HashSet::<&str>::new();
                set.insert(num[1]);
                graph.insert(num[0], set);
            },
        }
        match graph.get_mut(num[1]) {
            Some(v) => {
                v.insert(num[0]);
            },
            None => {
                let mut set = HashSet::<&str>::new();
                set.insert(num[0]);
                graph.insert(num[1], set);
            },
        }
    }

    let mut base = TreeNode::new();
    for (k, _) in graph.iter() {
        base.insert(k, &graph);
    }
    let (_, mut vector) = base.height();
    vector.sort();
    let res = vector.join(",");

    println!("Part 2: {}", res);
}

