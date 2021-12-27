use std::collections::HashMap;

fn part_one() -> i32 {
    play(8, 6)
}

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

fn part_two() -> u64 {
    let (ans1, ans2) = solve(&mut HashMap::new(), 7, 5, 0, 0);
    ans1.max(ans2)
}

fn solve(cache: &mut HashMap<(u8, u8, u8, u8), (u64, u64)>, p1: u8, p2: u8, score1: u8, score2: u8) -> (u64, u64) {
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
                let (u, v) = solve(cache, p2, p1, score2, score1);
                ans.0 += v;
                ans.1 += u;
            }
        }
    }

    cache.insert((p1, p2, score1, score2), ans);

    ans
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
