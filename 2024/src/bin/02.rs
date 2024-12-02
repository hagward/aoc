use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/02.txt").unwrap();

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let part1 = reports.iter().filter(|&levels| is_safe(levels)).count();

    let part2 = reports
        .iter()
        .filter(|&levels| {
            if is_safe(levels) {
                return true;
            }
            for i in 0..levels.len() {
                let mut levels_minus_one = levels.to_vec();
                levels_minus_one.remove(i);
                if is_safe(&levels_minus_one) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("{}", part1);
    println!("{}", part2);
}

fn is_safe(levels: &[i32]) -> bool {
    for i in 0..levels.len() - 1 {
        let a = levels[i];
        let b = levels[i + 1];
        if a.abs_diff(b) < 1 || a.abs_diff(b) > 3 {
            return false;
        }
        if i < levels.len() - 2 {
            let c = levels[i + 2];
            if (a - b).signum() != (b - c).signum() {
                return false;
            }
        }
    }
    true
}
