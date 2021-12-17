fn part_one(y1: i32) -> i32 {
    let d = -y1 - 1;
    d * (d + 1) / 2
}

fn part_two(x1: i32, x2: i32, y1: i32, y2: i32) -> usize {
    let mut valid_velocities = 0;
    for x_vel in 1..=x2 {
        for y_vel in y1..=(-y1 - 1) {
            if hits_target(x_vel, y_vel, (x1, x2, y1, y2)) {
                valid_velocities += 1;
            }
        }
    }
    valid_velocities
}

fn hits_target(start_x_vel: i32, start_y_vel: i32, (x1, x2, y1, y2): (i32, i32, i32, i32)) -> bool {
    let (mut x, mut y) = (0, 0);
    let (mut x_vel, mut y_vel) = (start_x_vel, start_y_vel);
    while x <= x2 && y >= y1 {
        x += x_vel;
        y += y_vel;
        x_vel = (x_vel - 1).max(0);
        y_vel -= 1;
        if x >= x1 && x <= x2 && y >= y1 && y <= y2 {
            return true;
        }
    }
    false
}

fn main() {
    let (x1, x2, y1, y2) = (144, 178, -100, -76);
    println!("Part one: {}", part_one(y1));
    println!("Part two: {}", part_two(x1, x2, y1, y2));
}
