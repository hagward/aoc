use std::fs;

static DIRS: [[i8; 2]; 4] = [[1, 0], [0, -1], [-1, 0], [0, 1]];

fn part_one(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    for line in input.trim().split("\n") {
        let action = &line[0..1];
        let units: &i64 = &line[1..].parse().unwrap();

        match action {
            "N" => y -= units,
            "S" => y += units,
            "E" => x += units,
            "W" => x -= units,
            "L" => dir = (dir + units).rem_euclid(360),
            "R" => dir = (dir - units).rem_euclid(360),
            "F" => {
                let [dx, dy] = DIRS[(dir / 90) as usize];
                x += dx as i64 * units;
                y += dy as i64 * units;
            }
            _ => println!("Unknown action '{}'", action),
        }
    }

    x.abs() + y.abs()
}

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    println!("{}", part_one(&input));
}
