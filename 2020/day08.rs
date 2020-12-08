use std::collections::HashSet;
use std::fs;

struct Computer {
    pc: i32,
    acc: i32,
    instr: Vec<Instruction>,
}

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Computer {
    fn run(&mut self) -> i32 {
        let mut visited = HashSet::new();
        while self.pc < self.instr.len() as i32 {
            if visited.contains(&self.pc) {
                return -1;
            }
            visited.insert(self.pc);
            self.run_instr();
        }
        0
    }

    fn run_instr(&mut self) {
        match &self.instr[self.pc as usize] {
            Instruction::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instruction::Jmp(n) => {
                self.pc += n;
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("day08.txt").unwrap();
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.trim().split("\n") {
        let tokens: Vec<&str> = line.split(" ").collect();
        let instr_name = tokens[0];
        let value = tokens[1].parse::<i32>().unwrap();
        let parsed_instr = match instr_name {
            "acc" => Instruction::Acc(value),
            "jmp" => Instruction::Jmp(value),
            "nop" => Instruction::Nop(value),
            _ => panic!("Unknown instruction '{}'", instr_name),
        };
        instructions.push(parsed_instr);
    }

    let mut computer = Computer {
        pc: 0,
        acc: 0,
        instr: instructions,
    };

    computer.run();
    println!("Accumulator stopped at: {}", computer.acc);

    for i in 0..computer.instr.len() {
        if flip_instr(&mut computer.instr, i) {
            computer.pc = 0;
            computer.acc = 0;
            let exit_code = computer.run();
            if exit_code == 0 {
                println!("Accumulator stopped at: {}", computer.acc);
                break;
            }
            flip_instr(&mut computer.instr, i);
        }
    }
}

fn flip_instr(instr: &mut Vec<Instruction>, i: usize) -> bool {
    match instr[i] {
        Instruction::Jmp(value) => {
            instr[i] = Instruction::Nop(value);
            true
        }
        Instruction::Nop(value) => {
            instr[i] = Instruction::Jmp(value);
            true
        }
        _ => false,
    }
}
