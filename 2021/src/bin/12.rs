use std::collections::{HashMap, HashSet};

static INPUT: &str = "zi-end\nXR-start\nzk-zi\nTS-zk\nzw-vl\nzk-zw\nend-po\nws-zw\nTS-ws\npo-TS\npo-YH\npo-xk\nzi-ws\nzk-end\nzi-XR\nXR-zk\nvl-TS\nstart-zw\nvl-start\nXR-zw\nXR-vl\nXR-ws";

fn part_one(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    dfs("start", nodes, HashSet::new())
}

fn part_two(nodes: &HashMap<&str, Vec<&str>>) -> usize {
    dfs2("start", nodes, HashMap::new(), false)
}

fn dfs<'a>(
    node: &'a str,
    nodes: &HashMap<&str, Vec<&str>>,
    mut visited: HashSet<&'a str>,
) -> usize {
    if node == "end" {
        return 1;
    }
    let mut num_paths = 0;
    let is_small_cave = node.chars().next().unwrap().is_lowercase();
    if !is_small_cave || visited.insert(node) {
        for neighbor in &nodes[node] {
            num_paths += dfs(&neighbor, nodes, visited.clone());
        }
    }
    num_paths
}

fn dfs2<'a>(
    node: &'a str,
    nodes: &HashMap<&str, Vec<&str>>,
    mut visited: HashMap<&'a str, usize>,
    is_twice: bool,
) -> usize {
    if node == "end" {
        return 1;
    }
    let mut num_paths = 0;
    let is_small_cave = node.chars().next().unwrap().is_lowercase();
    if !is_small_cave || *visited.entry(node).or_insert(0) < 2 {
        let mut is_twice = is_twice;
        if is_small_cave {
            let e = visited.get_mut(node).unwrap();
            *e += 1;
            if *e > 1 {
                if is_twice {
                    return 0;
                }
                is_twice = true;
            }
        }
        for neighbor in &nodes[node] {
            num_paths += dfs2(&neighbor, nodes, visited.clone(), is_twice);
        }
    }
    num_paths
}

fn main() {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    INPUT.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        if a != "end" && b != "start" {
            nodes.entry(a).or_insert(Vec::new()).push(b);
        }
        if a != "start" && b != "end" {
            nodes.entry(b).or_insert(Vec::new()).push(a);
        }
    });
    println!("Part one: {}", part_one(&nodes));
    println!("Part two: {}", part_two(&nodes));
}
