use std::fs;
use std::iter;

fn main() {
    let input = fs::read_to_string("src/bin/01.txt").unwrap();

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();
    left.sort();
    right.sort();

    let part1: u32 = iter::zip(left.clone(), right.clone())
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    let mut part2 = 0u32;
    for i in 0..left.len() {
        let mut times = 0u32;
        for j in 0..right.len() {
            if left[i] == right[j] {
                times += 1;
            } else if left[i] < right[j] {
                break;
            }
        }
        part2 += left[i] * times;
    }

    println!("{}", part1);
    println!("{}", part2);
}
