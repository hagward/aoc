use std::collections::HashSet;

static INPUT: &str = "6617113584\n6544218638\n5457331488\n1135675587\n1221353216\n1811124378\n1387864368\n4427637262\n6778645486\n3682146745";

fn part_one(mut input: Vec<Vec<u32>>) -> usize {
    (0..100).map(|_| step(&mut input)).sum()
}

fn part_two(mut input: Vec<Vec<u32>>) -> usize {
    let num_octopuses = input.len() * input[0].len();
    (1..).find(|_| step(&mut input) == num_octopuses).unwrap()
}

fn step(input: &mut Vec<Vec<u32>>) -> usize {
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            input[i][j] += 1;
        }
    }
    let mut flashed = HashSet::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            maybe_flash(&mut flashed, input, j, i);
        }
    }
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] > 9 {
                input[i][j] = 0;
            }
        }
    }
    flashed.len()
}

fn maybe_flash(
    flashed: &mut HashSet<(usize, usize)>,
    input: &mut Vec<Vec<u32>>,
    x: usize,
    y: usize,
) {
    if input[y][x] < 10 || !flashed.insert((x, y)) {
        return;
    }
    let d = [
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    for (dx, dy) in d {
        let xx = dx + x as i32;
        let yy = dy + y as i32;
        if xx < 0 || xx >= input[0].len() as i32 || yy < 0 || yy >= input.len() as i32 {
            continue;
        }
        let xx = xx as usize;
        let yy = yy as usize;
        input[yy][xx] += 1;
        maybe_flash(flashed, input, xx, yy);
    }
}

fn main() {
    let input: Vec<Vec<u32>> = INPUT
        .split_whitespace()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("Part one: {}", part_one(input.clone()));
    println!("Part two: {}", part_two(input));
}
