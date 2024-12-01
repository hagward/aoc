fn main() {
    let input = std::fs::read_to_string("src/bin/01.txt").unwrap();

    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            line.split_once("   ")
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
                .unwrap()
        })
        .unzip();
    left.sort();
    right.sort();

    let ans: u32 = std::iter::zip(left, right)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    println!("{}", ans);
}
