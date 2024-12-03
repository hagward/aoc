use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/03.txt").unwrap();
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

    let mut enable_mul = true;
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    re.captures_iter(&input)
        .for_each(|caps| match caps.get(0).unwrap().as_str() {
            "do()" => enable_mul = true,
            "don't()" => enable_mul = false,
            _ => {
                let a: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
                let b: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let product = a * b;
                part1 += product;
                if enable_mul {
                    part2 += product;
                }
            }
        });

    println!("{}", part1);
    println!("{}", part2);
}
