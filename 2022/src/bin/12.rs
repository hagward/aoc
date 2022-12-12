use std::collections::VecDeque;

// static INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";
static INPUT: &str = "abccccccccccccccccaaccccccccccccccccccccaaaaaaaaaaaaacccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaaa\nabcccccccccccccaaaaaccccccccccccccccccccaaaaaaaaaaaaaccccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaaa\nabccccccccccccccaaaaaccccccccccccccaaaaacccaaaaaacccccaaccccccccccccccccccccccccccccccccccccccccccccccccccccccaaaa\nabccccccccccccccaaaaacccccccccaacccaaaaacccaaaaaaaccccaaaacaacaaccccccccccccccccccccccccaaaccccaaaccccccccccccaaaa\nabcccccccccccccaaaaacccccccaaaaaccaaaaaacccaaaaaaaacaaaaaacaaaaaccccccccccccccccccccccccaaacccaaaaccccccccccccaaac\nabccccccaacaaaccccaaccccccccaaaaacaaaaaaccaaaacaaaacaaaaaccaaaaaaccccccccccccccccccccccccaaaaaaaacccccccccccccaacc\nabccccccaaaaaaccccccccccccccaaaaacaaaaaaccaaaaccaaaacaaaaacaaaaaacccccccccccccccccccccccaaaaaaaaaccccccccccccccccc\nabccccccaaaaaacccccccccccccaaaaaccccaaccccaacccccaaccaacaacaaaaaccccccccccccccccccccccccccaaakkkkllllcccaaaccccccc\nabccccccaaaaaaacccccccccccccccaaccccaacccccccccccccccccccccccaaaccccccaaaacccccccccjjjjkkkkkkkkkkllllccccaacaccccc\nabcccccaaaaaaaacccccaaccccccccccccccaaaaaaccccccccccccccccccaaccccccccaaaaccccccccjjjjjkkkkkkkkkppllllcccaaaaacccc\nabcccccaaaaaaaaccaaaacccccccccccccccaaaaaccccccccccccccccaacaaccccccccaaaacccccccjjjjjjjkkkkkppppppplllccaaaaacccc\nabccccccccaaaccccaaaaaacccccccccccaaaaaaaccccccccccccccccaaaaacccccccccaacccccccjjjjoooooooppppppppplllcccaaaccccc\nabccccccccaaccccccaaaaaccccaacccccaaaaaaaaccccaaacccccccccaaaaaaacccccccccccccccjjjooooooooppppuuppppllcccaaaccccc\nabccccccaacccccccaaaaacccccaaaccaaaaaaaaaaccaaaaaaccccccaaaaaaaaaacaaaccccccccccjjjoooouuuoopuuuuupppllcccaaaccccc\nabacccccaaccccccccccaacccccaaaaaaaccaaaaaaccaaaaaaccccccaaaaaccaaaaaaaccccaaccccjjoootuuuuuuuuuuuuvpqlllcccccccccc\nabaccaaaaaaaacccccccccccccccaaaaaaccaacccccccaaaaacccccccacaaaccaaaaaaccaaaacaccjjooottuuuuuuuxyuvvqqljjccddcccccc\nabcccaaaaaaaaccccccccccccaaaaaaaaacaacaaccccaaaaaccccccccccaaaaaaaaaacccaaaaaacciijootttxxxuuxyyyvvqqjjjjdddcccccc\nabcccccaaaaccccaaacccccccaaaaaaaaacaaaaaccccaaaaaccccccccccccaaaaaaaaacccaaaaccciiinntttxxxxxxyyvvqqqqjjjddddccccc\nabccccaaaaaccccaaaaacccccaaaaaaaaaaaaaaaaccccccccccccccccccccaaaaaaaaaaccaaaaccciiinntttxxxxxxyyvvvqqqqjjjdddccccc\nabccccaaaaaaccaaaaaccccccccaaaaaaaaaaaaaacccccccccccccccccccccccaaacaaacaacaaccciiinnnttxxxxxyyyvvvvqqqqjjjdddcccc\nSbccccaaccaaccaaaaacccccccccaaaaaaaaaaaaacccccccccccccccccccccccaaacccccccccccciiinnntttxxxEzzyyyyvvvqqqjjjdddcccc\nabcccccccccccccaaaaacccccccaaaaaaaaacaaaccccccccccccccccccccccccaaccccccccccccciiinnnttxxxxyyyyyyyyvvvqqqjjjdddccc\nabcccccccccccccaaccccccccccaaaaaaaaccccccccccccccccccccccccccccccccccccccccccciiinnntttxxyyyyyyyyyvvvvqqqjjjdddccc\nabccccccccccccccccccccccccaaaaaaaacccccccccccccccccccccccccccccccccccccccccccciiinntttxxxwwwyyywwvvvvrqqjjjjdddccc\nabcccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccciinnntttxwwwwwyyywwvvvrrrqkkkeddcccc\nabcccccccccccccccccccccccccccaaaaaaccccccccccccccccccccccccccccccccccccccccccchhnnntttsswwswwyywwrrrrrrkkkkeeecccc\nabcccccccccccccccccccccccccccaaaaaacccccccccccccccccccaccccccccccccaaacccccccchhhnmmssssssswwwwwwrrrkkkkkeeeeecccc\nabcccccccccccccccccccccccccccccaaacccccccccccccccccccaaccccccccccaaaaaacccccaahhhmmmmmsssssswwwwrrrkkkkkeeeeeccccc\nabaacccccccccccccaccccccccccccccccccccccccccccccccaaaaacaacccccccaaaaaacaaaaaahhhhmmmmmmmmssswwwrrkkkkeeeeeacccccc\nabacccccccccccccaaaaaaaaccccccccaaacccccccaaccccccaaaaaaaacccccccaaaaaacaaaaaaahhhhmmmmmmmmsssrrrrkkkeeeeeaacccccc\nabaaaccccaaccccccaaaaaacccccccccaaacccaacaaaccccccccaaaacccccccccaaaaacccaaaaaaahhhhhhhmmmmlsssrrllkfeeeeaaaaacccc\nabaaaccaaaaccccccaaaaaacccccccccaaaaaaaaaaaaaacccccaaaaacccccccccaaaaacccaaaaaaachhhhhgggmllsssrrllkffeaaaaaaacccc\nabaacccaaaaaacccaaaaaaaacccccaaaaaaaaaaaaaaaaacccccaacaaacccccccccccccccaaaaaacccccchggggglllllllllfffaaaaaaaacccc\nabaaccccaaaacccaaaaaaaaaaccccaaaaaaaaacaaaaaaaccaccaccaaacccccccccccccccaaaaaacccccccccgggglllllllffffaaaaaacccccc\nabcccccaaaaacccaaaaaaaaaacccccaaaaaaaccaaaaacccaaaccccccccccccccccccccccccccaacccccccccagggglllllffffccccaaacccccc\nabcccccaacaaccccccaaaaacaccaacccaaaaaaaaaaaaaccaaacccccccccccccccccccccccccccccccccccccaagggggffffffcccccccccccccc\nabcccccccccccaaaaaaaaacccccaaccaaaaaaaccaaaaacaaaaccccccccccccccccccccccccccccccccccccaaaacgggfffffccccccccccccccc\nabcccccccccccaaaaacaacccaaaaaaaaaaccaacccaaaaaaaacccaaccccccccccccccccccccccccccccccccccccccggfffccccccccccccaaaca\nabccccccccccaaaaaaccccccaaaaaaaaacccccccccaaaaaaaaaaaacccccccccccccaaaccccccccccccccccccccccaaaccccccccccccccaaaaa\nabccccccccccaaaaaaccccccccaaaacccccccccccccaaaaaaaaaaaaccccccccccccaaaaccccccccccccccccccccccaaaccccccccccccccaaaa\nabcccccccccccaaaaacccccccaaaaaaccccccccccaaaaaaaaaaaaaaccccccccccccaaaaccccccccccccccccccccccccccccccccccccccaaaaa";

fn main() {
    let input: Vec<&[u8]> = INPUT
        .split_ascii_whitespace()
        .map(|line| line.as_bytes())
        .collect();

    let mut start_i = 0;
    let mut start_j = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == b'S' {
                start_i = i;
                start_j = j;
                break;
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_i, start_j));

    let mut distance = vec![vec![-1; input[0].len()]; input.len()];
    distance[start_i][start_j] = 0;

    while let Some((i, j)) = queue.pop_front() {
        if input[i][j] == b'E' {
            println!("Found E in {} steps", distance[i][j]);
            break;
        }

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let i2 = i as i32 + dy;
            let j2 = j as i32 + dx;
            if i2 < 0 || i2 >= input.len() as i32 || j2 < 0 || j2 >= input[0].len() as i32 {
                continue;
            }
            let i2 = i2 as usize;
            let j2 = j2 as usize;
            let a = if input[i][j] == b'S' {
                b'a'
            } else {
                input[i][j]
            };
            let b = if input[i2][j2] == b'E' {
                b'z'
            } else {
                input[i2][j2]
            };
            if b as i32 - a as i32 > 1 {
                continue;
            }
            if distance[i2][j2] != -1 {
                continue;
            }
            distance[i2][j2] = distance[i][j] + 1;
            queue.push_back((i2, j2));
        }
    }
}
