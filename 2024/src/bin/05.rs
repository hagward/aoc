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
    let error_indexes = |update: &[&str]| -> Option<(usize, usize)> {
        for i in 0..update.len() - 1 {
            for j in i + 1..update.len() {
                if map
                    .get(update[j])
                    .map(|set| set.contains(update[i]))
                    .unwrap_or_default()
                {
                    return Some((i, j));
                }
            }
        }
        None
    };
    let mut sum_part1 = 0;
    updates.lines().for_each(|update| {
        let update: Vec<&str> = update.split(",").collect();
        if error_indexes(&update).is_none() {
            let mid: usize = update[update.len() / 2].parse().unwrap();
            sum_part1 += mid;
        }
    });
    println!("{}", sum_part1);

    let mut sum_part2 = 0;
    updates.lines().for_each(|update| {
        let mut update: Vec<&str> = update.split(",").collect();
        let mut had_error = false;
        while let Some((i, j)) = error_indexes(&update) {
            (update[i], update[j]) = (update[j], update[i]);
            had_error = true;
        }
        if had_error {
            let mid: usize = update[update.len() / 2].parse().unwrap();
            sum_part2 += mid;
        }
    });
    println!("{}", sum_part2);
}
