use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("day11-input.txt");
    let mut file = match File::open(&path) {
        Err(why) => panic!("{}", why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => print_distance(&s),
    }
}

fn print_distance(input: &str) {
    let v: Vec<&str> = input.split(',').collect();

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut max_distance = 0;

    for step in v.iter() {
        match step.trim() {
            "n"  => { y += 1; z -= 1 },
            "ne" => { x += 1; z -= 1 },
            "se" => { x += 1; y -= 1 },
            "s"  => { y -= 1; z += 1 },
            "sw" => { x -= 1; z += 1 },
            "nw" => { x -= 1; y += 1 },
            unknown => println!("Unknown input: {}", unknown),
        }
        let d = cube_distance((0, 0, 0), (x, y, z));
        if d > max_distance {
            max_distance = d;
        }
    }

    println!("({}, {}, {})", x, y, z);
    println!("Final distance: {}", cube_distance((0, 0, 0), (x, y, z)));
    println!("Max distance: {}", max_distance);
}

fn cube_distance(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> i64 {
    ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs()) / 2
}
