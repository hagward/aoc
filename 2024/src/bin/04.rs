use std::fs;

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("src/bin/04.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut part_1 = 0;
    let mut part_2 = 0;
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
                    part_1 += 1;
                }
            }

            if (has_mas(&input, x, y, 1, 1) || has_mas(&input, x + 2, y + 2, -1, -1))
                && (has_mas(&input, x + 2, y, -1, 1) || has_mas(&input, x, y + 2, 1, -1))
            {
                part_2 += 1;
            }
        }
    }

    println!("{}", part_1);
    println!("{}", part_2);
}

fn has_xmas(input: &[Vec<char>], xs: usize, ys: usize, dx: i32, dy: i32) -> bool {
    let needle = &['X', 'M', 'A', 'S'];
    has_string(needle, input, xs, ys, dx, dy)
}

fn has_mas(input: &[Vec<char>], xs: usize, ys: usize, dx: i32, dy: i32) -> bool {
    let needle = &['M', 'A', 'S'];
    has_string(needle, input, xs, ys, dx, dy)
}

fn has_string(s: &[char], input: &[Vec<char>], xs: usize, ys: usize, dx: i32, dy: i32) -> bool {
    for i in 0..s.len() {
        let x = (xs as i32 + dx * i as i32) as usize;
        let y = (ys as i32 + dy * i as i32) as usize;
        if y >= input.len() || x >= input[y].len() || input[y][x] != s[i] {
            return false;
        }
    }
    true
}
