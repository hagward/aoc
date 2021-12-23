fn play(p1_start: i32, p2_start: i32) -> i32 {
    let (mut a_index, mut b_index) = (p1_start - 1, p2_start - 1);
    let (mut a_score, mut b_score) = (0, 0);
    let mut next_die_roll = 1;
    loop {
        a_index = (a_index + 3 * next_die_roll + 3) % 10;
        a_score += a_index + 1;
        next_die_roll += 3;
        if a_score >= 1000 {
            break;
        }
        b_index = (b_index + 3 * next_die_roll + 3) % 10;
        b_score += b_index + 1;
        next_die_roll += 3;
        if b_score >= 1000 {
            break;
        }
    }
    a_score.min(b_score) * (next_die_roll - 1)
}

fn main() {
    println!("Part one: {}", play(8, 6));
}
