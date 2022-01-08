use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};

static INPUT: &str = "#############\n#...........#\n###B#B#D#D###\n  #C#A#A#C#  \n  #########  ";

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
        let mut queue = VecDeque::from([(xs, ys, 0_usize)]);
        let mut visited = vec![vec![false; self.board[0].len()]; self.board.len()];
        visited[ys][xs] = true;
        while !queue.is_empty() {
            let (x, y, steps) = queue.pop_front().unwrap();

            if steps > 0
                && (x != 3 || y != 1)
                && (x != 5 || y != 1)
                && (x != 7 || y != 1)
                && (x != 9 || y != 1)
                && (x != 3
                    || y != 2
                    || amph == 'A' && (self.board[3][3] == amph)
                    || self.board[3][3] == '.')
                && (x != 5
                    || y != 2
                    || amph == 'B' && (self.board[3][5] == amph)
                    || self.board[3][5] == '.')
                && (x != 7
                    || y != 2
                    || amph == 'C' && (self.board[3][7] == amph)
                    || self.board[3][7] == '.')
                && (x != 9
                    || y != 2
                    || amph == 'D' && (self.board[3][9] == amph)
                    || self.board[3][9] == '.')
            {
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

    fn is_goal(&self) -> bool {
        let b = &self.board;
        b[2][3] == 'A'
            && b[3][3] == 'A'
            && b[2][5] == 'B'
            && b[3][5] == 'B'
            && b[2][7] == 'C'
            && b[3][7] == 'C'
            && b[2][9] == 'D'
            && b[3][9] == 'D'
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

fn shortest_path(start_state: State) -> Option<usize> {
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
    let board: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let state = State { board, cost: 0 };
    println!("Part one: {}", shortest_path(state).unwrap());
}
