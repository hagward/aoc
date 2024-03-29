use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

static INPUT_P1: &str = "#############\n#...........#\n###B#B#D#D###\n  #C#A#A#C#  \n  #########  ";
static INPUT_P2: &str = "#############\n#...........#\n###B#B#D#D###\n  #D#C#B#A#  \n  #D#B#A#C#  \n  #C#A#A#C#  \n  #########  ";

#[derive(Clone, Eq, PartialEq)]
struct State {
    board: Vec<Vec<char>>,
    cost: usize,
}

impl State {
    fn key(&self) -> u128 {
        let mut key: u128 = 0;
        for y in 1..self.board.len() - 1 {
            for x in 1..self.board[y].len() - 1 {
                let c = self.board[y][x];
                if c == '.' {
                    key = (key << 3) | 7;
                } else if c.is_ascii_alphabetic() {
                    key = (key << 3) | (c as u8 - b'A') as u128;
                }
            }
        }
        key
    }

    fn all_moves(&self) -> Vec<State> {
        let mut moves = Vec::new();
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x].is_ascii_alphabetic() {
                    moves.append(&mut self.moves(x, y));
                }
            }
        }
        moves
    }

    fn moves(&self, xs: usize, ys: usize) -> Vec<State> {
        let mut moves = Vec::new();
        let amph = self.board[ys][xs];

        if self.is_amph_settled(xs, ys) {
            return Vec::new();
        }

        let mut queue = VecDeque::from([(xs, ys, 0_usize)]);
        let mut visited = vec![vec![false; self.board[0].len()]; self.board.len()];
        visited[ys][xs] = true;
        while !queue.is_empty() {
            let (x, y, steps) = queue.pop_front().unwrap();

            if steps > 0 && self.is_valid_move(amph, ys, x, y) {
                let cost = 10_usize.pow(amph as u32 - 'A' as u32) * steps;
                let mut new_state = self.clone();
                new_state.board[ys][xs] = '.';
                new_state.board[y][x] = amph;
                new_state.cost += cost;
                moves.push(new_state);
            }

            for (xn, yn) in [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)] {
                if self.board[yn][xn] != '.' || visited[yn][xn] {
                    continue;
                }
                visited[yn][xn] = true;
                queue.push_back((xn, yn, steps + 1));
            }
        }
        moves
    }

    fn is_amph_settled(&self, x: usize, y: usize) -> bool {
        if y == 1 {
            return false;
        }

        // Wrong corridor.
        let amph = self.board[y][x];
        if amph as u8 - b'A' != ((x - 3) / 2) as u8 {
            return false;
        }

        // Another amph wants to get out.
        let next_amph = self.board[y + 1][x];
        if amph != next_amph && next_amph != '#' {
            return false;
        }

        true
    }

    fn is_valid_move(&self, amph: char, ys: usize, x: usize, y: usize) -> bool {
        // Cannot move from hallway to another place in hallway.
        if ys == 1 && y == 1 {
            return false;
        }

        // Cannot stop in front of a corridor.
        if y == 1 && x > 2 && x < self.board[y].len() - 2 && x % 2 == 1 {
            return false;
        }

        if y > 1 {
            // Cannot enter another amphipod's corridor.
            if amph as u8 - b'A' != ((x - 3) / 2) as u8 {
                return false;
            }

            // Cannot enter a corridor where there are amphipods that want to get out.
            let c = self.board[y + 1][x];
            if c != amph && c != '#' {
                return false;
            }
        }

        true
    }

    fn is_goal(&self) -> bool {
        for y in 2..self.board.len() - 1 {
            for x in 0..4 {
                let c = (b'A' + x as u8) as char;
                let x = x * 2 + 3;
                if self.board[y][x] != c {
                    return false;
                }
            }
        }
        true
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(input: &str) -> Option<usize> {
    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_state = State { board, cost: 0 };
    let mut dist: HashMap<u128, usize> = HashMap::from([(start_state.key(), 0)]);
    let mut heap = BinaryHeap::from([start_state]);

    while let Some(state) = heap.pop() {
        if state.is_goal() {
            return Some(state.cost);
        }

        let key = state.key();
        if state.cost > *dist.get(&key).unwrap_or(&usize::MAX) {
            continue;
        }

        for new_state in state.all_moves() {
            let new_cost = new_state.cost;
            let new_key = new_state.key();
            if new_cost < *dist.get(&new_key).unwrap_or(&usize::MAX) {
                heap.push(new_state);
                dist.insert(new_key, new_cost);
            }
        }
    }

    None
}

fn main() {
    println!("Part one: {}", solve(INPUT_P1).unwrap());
    println!("Part two: {}", solve(INPUT_P2).unwrap());
}
