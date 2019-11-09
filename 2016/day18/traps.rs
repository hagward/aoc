// . = 46
// ^ = 94

// fn main() {
//     let width: i32 = 100;
//     let height: i32 = 40;
// 
//     let s = String::from(".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^");
// 
//     let mut v: Vec<Vec<u8>> = Vec::new();
//     v.push(s.into_bytes());
// 
//     for i in 1..height {
//         v.push(Vec::new());
// 
//         for j in 0..width {
//             let left = is_trap(j-1, i-1, &v);
//             let center = is_trap(j, i-1, &v);
//             let right = is_trap(j+1, i-1, &v);
// 
//             if left && center && !right ||
//                 !left && center && right ||
//                 left && !center && !right ||
//                 !left && !center && right {
// 
//                 v[i as usize].push(94);
//             } else {
//                 v[i as usize].push(46);
//             }
//         }
//     }
// 
//     print!("Safe tiles: {}\n", num_safe_tiles(&v));
//     print!("{}", vec_to_str(&v));
// }

// fn vec_to_str(v: &Vec<Vec<u8>>) -> String {
//     let mut s = String::new();
//     for i in 0..v.len() {
//         let line = String::from_utf8(v[i].clone());
//         s.push_str(line.unwrap().as_str());
//         s.push_str("\n");
//     }
//     s
// }

// fn num_safe_tiles(v: &Vec<Vec<u8>>) -> u32 {
//     let mut n = 0;
//     for i in 0..v.len() {
//         for j in 0..v[i].len() {
//             if v[i][j] == 46 {
//                 n += 1;
//             }
//         }
//     }
//     n
// }

const WIDTH: usize = 100;
const HEIGHT: usize = 400000;

fn main() {
    let safe_tiles = num_safe_tiles(".^.^..^......^^^^^...^^^...^...^....^^.^...^.^^^^....^...^^.^^^...^^^^.^^.^.^^..^.^^^..^^^^^^.^^^..^");
    print!("{}", safe_tiles);
}

fn num_safe_tiles(s: &str) -> u32 {
    let mut safe_tiles = 0;
    let ss = String::from(s);

    let bytes = ss.as_bytes();
    let mut b1 = [46; WIDTH];

    for i in 0..WIDTH {
        b1[i] = bytes[i];
        if !is_trap(i as i32, &bytes) {
            safe_tiles += 1;
        }
    }

    for _ in 1..HEIGHT {
        let mut b2: [u8; WIDTH] = [94; WIDTH];
        for j in 0..WIDTH {
            if !is_new_trap(j, &b1) {
                b2[j] = 46;
                safe_tiles += 1;
            }
        }
        for j in 0..WIDTH {
            b1[j] = b2[j];
        }
    }

    safe_tiles
}

fn is_new_trap(i: usize, bytes: &[u8]) -> bool {
    let i = i as i32;
    let left = is_trap(i-1, bytes);
    let center = is_trap(i, bytes);
    let right = is_trap(i+1, bytes);

    left && center && !right ||
        !left && center && right ||
        left && !center && !right ||
        !left && !center && right
}

fn is_trap(i: i32, bytes: &[u8]) -> bool {
    if i < 0 || i >= bytes.len() as i32 {
        false
    } else {
        bytes[i as usize] == 94
    }
}

// fn is_trap(x: i32, y: i32, v: &Vec<Vec<u8>>) -> bool {
//     if x < 0 || x >= v[0].len() as i32 {
//         return false;
//     } else {
//         return v[y as usize][x as usize] == 94;
//     }
// }

