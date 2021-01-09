use std::fs;

fn part_one(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 1;
    let mut dy = 0;

    for line in input.trim().split("\n") {
        let action = &line[0..1];
        let units: i64 = line[1..].parse().unwrap();

        match action {
            "N" => y += units,
            "S" => y -= units,
            "E" => x += units,
            "W" => x -= units,
            "L" | "R" => {
                let (new_dx, new_dy) = rotate(action, units, dx, dy);
                dx = new_dx;
                dy = new_dy;
            }
            "F" => {
                x += dx * units;
                y += dy * units;
            }
            _ => panic!("Unknown action '{}'", action),
        }
    }

    x.abs() + y.abs()
}

fn part_two(input: &str) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 10;
    let mut dy = 1;

    for line in input.trim().split("\n") {
        let action = &line[0..1];
        let units: i64 = line[1..].parse().unwrap();

        match action {
            "N" => dy += units,
            "S" => dy -= units,
            "E" => dx += units,
            "W" => dx -= units,
            "L" | "R" => {
                let (new_dx, new_dy) = rotate(action, units, dx, dy);
                dx = new_dx;
                dy = new_dy;
            }
            "F" => {
                x += dx * units;
                y += dy * units;
            }
            _ => panic!("Unknown action '{}'", action),
        }
    }

    x.abs() + y.abs()
}

fn rotate(dir: &str, deg: i64, x: i64, y: i64) -> (i64, i64) {
    match (dir, deg) {
        ("L", 90) => (-y, x),
        ("L", 180) => (-x, -y),
        ("L", 270) => (y, -x),
        ("R", 90) => (y, -x),
        ("R", 180) => (-x, -y),
        ("R", 270) => (-y, x),
        (_, _) => panic!("Unknown rotation '{}{}'", dir, deg),
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day12.txt").unwrap();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), String> {
        let result = part_one("F10\nN3\nF7\nR90\nF11");
        assert_eq!(result, 25);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), String> {
        let result = part_two("F10\nN3\nF7\nR90\nF11");
        assert_eq!(result, 286);
        Ok(())
    }
}
