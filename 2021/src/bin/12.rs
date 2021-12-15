use std::collections::{HashMap, HashSet};

static INPUT: &str = "zi-end\nXR-start\nzk-zi\nTS-zk\nzw-vl\nzk-zw\nend-po\nws-zw\nTS-ws\npo-TS\npo-YH\npo-xk\nzi-ws\nzk-end\nzi-XR\nXR-zk\nvl-TS\nstart-zw\nvl-start\nXR-zw\nXR-vl\nXR-ws";

fn dfs<'a>(
    node: &'a str,
    nodes: &HashMap<&str, Vec<&str>>,
    mut visited: HashSet<&'a str>,
    is_twice: bool,
    is_part2: bool,
) -> usize {
    if node == "end" {
        return 1;
    }
    let mut num_paths = 0;
    let is_small_cave = node.chars().next().unwrap().is_lowercase();
    if !is_small_cave || visited.insert(node) {
        for neighbor in &nodes[node] {
            num_paths += dfs(neighbor, nodes, visited.clone(), is_twice, is_part2);
        }
    } else if is_part2 && !is_twice && node != "start" {
        for neighbor in &nodes[node] {
            num_paths += dfs(neighbor, nodes, visited.clone(), true, true);
        }
    }
    num_paths
}

fn main() {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    INPUT.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        nodes.entry(a).or_insert_with(Vec::new).push(b);
        nodes.entry(b).or_insert_with(Vec::new).push(a);
    });
    let part1 = dfs("start", &nodes, HashSet::new(), false, false);
    let part2 = dfs("start", &nodes, HashSet::new(), false, true);
    println!("Part one: {}", part1);
    println!("Part two: {}", part2);
}
