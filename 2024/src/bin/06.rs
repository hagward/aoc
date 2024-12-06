use std::{collections::HashSet, fs};

static DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let input = fs::read_to_string("src/bin/06.txt").unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (ys, xs) = find('^', &grid);
    grid[ys as usize][xs as usize] = 'X';

    println!("{}", part1(ys, xs, grid.clone()));
    println!("{}", part2(ys, xs, grid.clone()));
}

fn find(c: char, grid: &[Vec<char>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == c {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn part1(ys: usize, xs: usize, mut grid: Vec<Vec<char>>) -> usize {
    let mut y = ys as i32;
    let mut x = xs as i32;
    let mut di = 0;
    let mut positions = 1;
    loop {
        let (dy, dx) = DIRECTIONS[di];
        let (y2, x2) = (y + dy, x + dx);
        if y2 < 0 || y2 >= grid.len() as i32 || x2 < 0 || x2 >= grid[0].len() as i32 {
            return positions;
        }
        if grid[y2 as usize][x2 as usize] == '#' {
            di = (di + 1) % DIRECTIONS.len();
        } else {
            (y, x) = (y2, x2);
            if grid[y as usize][x as usize] == '.' {
                positions += 1;
            }
            grid[y as usize][x as usize] = 'X';
        }
    }
}

fn part2(ys: usize, xs: usize, mut grid: Vec<Vec<char>>) -> usize {
    let mut loops = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '.' {
                grid[i][j] = '#';
                if has_loop(ys, xs, &grid) {
                    loops += 1;
                }
                grid[i][j] = '.';
            }
        }
    }
    loops
}

fn has_loop(ys: usize, xs: usize, grid: &[Vec<char>]) -> bool {
    let mut states = HashSet::<(usize, usize, i32, i32)>::new();
    states.insert((ys, xs, -1, 0));
    let mut y = ys as i32;
    let mut x = xs as i32;
    let mut di = 0;
    loop {
        let (dy, dx) = DIRECTIONS[di];
        let (y2, x2) = (y + dy, x + dx);
        if y2 < 0 || y2 >= grid.len() as i32 || x2 < 0 || x2 >= grid[0].len() as i32 {
            return false;
        }
        if states.contains(&(y2 as usize, x2 as usize, dy, dx)) {
            return true;
        }
        if grid[y2 as usize][x2 as usize] == '#' {
            di = (di + 1) % DIRECTIONS.len();
        } else {
            (y, x) = (y2, x2);
            states.insert((y as usize, x as usize, dy, dx));
        }
    }
}
