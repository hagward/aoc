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

fn main() {
    let result = part_one(TIMESTAMP, TIMETABLE);
    println!("{}", result);
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
}
