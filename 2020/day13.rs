static TIMESTAMP: u64 = 1001796;
static TIMETABLE: &str = "37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,457,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,23,x,x,x,x,x,29,x,431,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19";

fn part_one(timestamp: u64, timetable: &str) -> u64 {
    timetable
        .split(",")
        .filter(|&id_str| id_str != "x")
        .map(|id_str| {
            let id: u64 = id_str.parse().unwrap();
            (id, id - (timestamp % id))
        })
        .min_by_key(|&(_, wait)| wait)
        .map(|(id, wait)| id * wait)
        .unwrap()
}

fn part_two(timetable: &str) -> u64 {
    let buses: Vec<(u64, u64)> = timetable
        .split(",")
        .enumerate()
        .filter(|&(_, id_str)| id_str != "x")
        .map(|(i, id_str)| (i as u64, id_str.parse::<u64>().unwrap()))
        .collect();

    let (mut t, mut step) = buses[0];
    for (delta, period) in buses.iter().skip(1) {
        while (t + delta) % period != 0 {
            t += step;
        }
        step = lcm(step, *period);
    }

    t
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    println!("{}", part_one(TIMESTAMP, TIMETABLE));
    println!("{}", part_two(TIMETABLE));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() -> Result<(), String> {
        let result = part_one(939, "7,13,x,x,59,x,31,19");
        assert_eq!(result, 295);
        Ok(())
    }

    #[test]
    fn test_part_two() -> Result<(), String> {
        let result = part_two("7,13,x,x,59,x,31,19");
        assert_eq!(result, 1068781);
        Ok(())
    }
}
