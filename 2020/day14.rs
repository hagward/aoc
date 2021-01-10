use std::collections::HashMap;
use std::fs;

fn part_one(input: &str) -> u64 {
    let mut mem = HashMap::new();
    let mut zero_mask = 0;
    let mut one_mask = 0;

    for line in input.trim().split("\n") {
        if line.starts_with("mask") {
            let (m1, m2) = parse_mask(&line[7..]);
            zero_mask = m1;
            one_mask = m2;
        } else {
            let bracket_end = line.find(']').unwrap();
            let value_start = 8 + bracket_end - 4;
            let address: usize = line[4..bracket_end].parse().unwrap();
            let value: u64 = line[value_start..].parse().unwrap();

            mem.insert(address, (value & zero_mask) | one_mask);
        }
    }

    mem.values().sum()
}

fn parse_mask(mask: &str) -> (u64, u64) {
    let mut zero_mask = 0;
    let mut one_mask = 0;

    for c in mask.chars() {
        zero_mask <<= 1;
        one_mask <<= 1;

        if c != '0' {
            zero_mask |= 1;
        }
        if c == '1' {
            one_mask |= 1;
        }
    }

    (zero_mask, one_mask)
}

fn main() {
    let input = fs::read_to_string("inputs/day14.txt").unwrap();
    println!("{}", part_one(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), String> {
        let result = part_one(
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0",
        );
        assert_eq!(result, 165);
        Ok(())
    }
}
