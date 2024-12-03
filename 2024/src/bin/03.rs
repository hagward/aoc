use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/03.txt").unwrap();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let part1: i32 = re
        .captures_iter(&input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum();

    println!("{}", part1);
}
