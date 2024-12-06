use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let binding = fs::read_to_string("src/bin/05.txt").unwrap();
    let (rules, updates) = binding.split_once("\n\n").unwrap();
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    rules.lines().for_each(|rule| {
        let (l, r) = rule.split_once("|").unwrap();
        map.entry(l).or_insert_with(|| HashSet::new()).insert(r);
    });
    let is_correct = |update: &[&str]| {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if map.contains_key(update[j]) && map.get(update[j]).unwrap().contains(update[i]) {
                    return false;
                }
            }
        }
        true
    };
    let mut sum = 0;
    updates.lines().for_each(|update| {
        let update: Vec<&str> = update.split(",").collect();
        if is_correct(&update) {
            let mid: usize = update[update.len() / 2].parse().unwrap();
            sum += mid;
        }
    });
    println!("{}", sum);
}
