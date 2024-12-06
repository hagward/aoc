use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/06.txt").unwrap();

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (ys, xs) = find('^', &grid);
    grid[ys as usize][xs as usize] = 'X';

    let mut x = xs as i32;
    let mut y = ys as i32;
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut di = 0;
    let mut part1 = 1;
    loop {
        let (dy, dx) = dirs[di];
        let x2 = x + dx;
        let y2 = y + dy;
        if x2 < 0 || x2 >= grid[0].len() as i32 || y2 < 0 || y2 >= grid.len() as i32 {
            break;
        }
        if grid[y2 as usize][x2 as usize] == '#' {
            di = (di + 1) % dirs.len();
        } else {
            x = x2;
            y = y2;
            if grid[y as usize][x as usize] != 'X' {
                part1 += 1;
            }
            grid[y as usize][x as usize] = 'X';
        }
    }

    // for line in grid {
    //     for c in line {
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    println!("{}", part1);
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
