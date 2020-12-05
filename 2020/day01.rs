use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let input: Vec<i32> = lines_from_file("day01.txt")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let a = input[i];
            let b = input[j];
            if a + b == 2020 {
                println!("{} + {} = 2020", a, b);
                println!("{} * {} = {}", a, b, a * b);
            }
            for k in j + 1..input.len() {
                let c = input[k];
                if a + b + c == 2020 {
                    println!("{} + {} + {} = 2020", a, b, c);
                    println!("{} * {} * {} = {}", a, b, c, a * b * c);
                    return;
                }
            }
        }
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
