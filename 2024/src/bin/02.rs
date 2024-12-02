use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/02.txt").unwrap();

    let part1 = input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_ascii_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            for i in 0..numbers.len() - 1 {
                let a = numbers[i];
                let b = numbers[i + 1];
                if a.abs_diff(b) < 1 || a.abs_diff(b) > 3 {
                    return false;
                }
                if i < numbers.len() - 2 {
                    let c = numbers[i + 2];
                    if (a - b).signum() != (b - c).signum() {
                        return false;
                    }
                }
            }
            true
        })
        .count();

    println!("{}", part1);
}
