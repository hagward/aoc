use std::env;

fn brute_force(n: usize) -> usize {
    let mut elves_left = n;
    let mut p = vec![true; n];
    let mut i = 0;

    while elves_left > 1 {
        i = (i + 1) % n;
        while !p[i] {
            i = (i + 1) % n;
        }
        p[i] = false;
        elves_left -= 1;
        while !p[i] {
            i = (i + 1) % n;
        }
    }

    i + 1
}

fn josephus(n: u64, k: u64) -> u64 {
    let n = n as f64;
    let k = k as f64;

    let a = n.log(k).floor();
    let l = n - k.powf(a);

    print!("n: {}, k: {}, a: {}, l: {}\n", n, k, a, l);

    (((k * l) % n) + 1.0) as u64
}

fn josephus_rec(n: u64, k: u64) -> u64 {
    if n == 1 {
        return 0;
    } else {
        return (josephus_rec(n - 1, k) + k) % n;
    }
}

fn josephus_dyn(n: u64, k: u64) -> u64 {
    let mut d: Vec<u64> = Vec::new();
    d.push(0);
    d.push(0);

    for i in 2..n {
        let j: usize = (i - 1) as usize;
        let last = d[j];
        d.push((last + k) % n);
    }

    return *d.last().unwrap();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let n: u64 = if args.len() > 1 { args[1].parse().unwrap() } else { 3018458 };
    let k: u64 = if args.len() > 2 { args[2].parse().unwrap() } else { 2 };

    print!("n: {}, k: {}\n", n, k);

    print!("{}\n", josephus_dyn(n, k) + 1);
}

