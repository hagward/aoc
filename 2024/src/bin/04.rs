use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("src/bin/04.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for (dx, dy) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ] {
                if has_xmas(&input, x, y, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}

fn has_xmas(input: &[Vec<char>], start_x: usize, start_y: usize, dx: i32, dy: i32) -> bool {
    let xmas = &['X', 'M', 'A', 'S'];
    for i in 0..4 {
        let x = (start_x as i32 + dx * i as i32) as usize;
        let y = (start_y as i32 + dy * i as i32) as usize;
        if y >= input.len() || x >= input[y].len() || input[y][x] != xmas[i] {
            return false;
        }
    }
    true
}
