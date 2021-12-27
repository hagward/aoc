use std::collections::HashMap;

fn part_one(p1_start: usize, p2_start: usize) -> usize {
    let mut indexes = [p1_start - 1, p2_start - 1];
    let mut scores = [0, 0];
    let mut next_roll = 1;
    let mut turn = 0;
    loop {
        indexes[turn] = (indexes[turn] + 3 * next_roll + 3) % 10;
        scores[turn] += indexes[turn] + 1;
        next_roll += 3;
        if scores[turn] >= 1000 {
            break;
        }
        turn = (turn + 1) % 2;
    }
    scores.iter().min().unwrap() * (next_roll - 1)
}

fn part_two(p1_start: usize, p2_start: usize) -> u64 {
    let (ans1, ans2) = solve_quantum(
        &mut HashMap::new(),
        (p1_start - 1) as u8,
        (p2_start - 1) as u8,
        0,
        0,
    );
    ans1.max(ans2)
}

fn solve_quantum(
    cache: &mut HashMap<(u8, u8, u8, u8), (u64, u64)>,
    p1: u8,
    p2: u8,
    score1: u8,
    score2: u8,
) -> (u64, u64) {
    if score1 >= 21 {
        return (1, 0);
    }
    if score2 >= 21 {
        return (0, 1);
    }
    if let Some(ans) = cache.get(&(p1, p2, score1, score2)) {
        return *ans;
    }

    let mut ans = (0, 0);
    for roll1 in 1..4 {
        for roll2 in 1..4 {
            for roll3 in 1..4 {
                let p1 = (p1 + roll1 + roll2 + roll3) % 10;
                let score1 = score1 + p1 + 1;
                let (ans1, ans2) = solve_quantum(cache, p2, p1, score2, score1);
                ans.0 += ans2;
                ans.1 += ans1;
            }
        }
    }
    cache.insert((p1, p2, score1, score2), ans);
    ans
}

fn main() {
    let (p1_start, p2_start) = (8, 6);
    println!("Part one: {}", part_one(p1_start, p2_start));
    println!("Part one: {}", part_two(p1_start, p2_start));
}
