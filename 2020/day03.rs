use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").unwrap();
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let trees11 = check_slope(&lines, 1, 1);
    let trees31 = check_slope(&lines, 3, 1);
    let trees51 = check_slope(&lines, 5, 1);
    let trees71 = check_slope(&lines, 7, 1);
    let trees12 = check_slope(&lines, 1, 2);

    println!("Part 1: {}", trees31);
    println!(
        "Part 2: {}",
        trees11 * trees31 * trees51 * trees71 * trees12
    );
}

fn check_slope(lines: &Vec<&str>, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = down;

    while y < lines.len() {
        let line = lines[y];

        x = (x + right) % line.len();
        y += down;

        if line.chars().nth(x).unwrap() == '#' {
            trees += 1;
        }
    }

    trees
}
