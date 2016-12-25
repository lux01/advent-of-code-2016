use std::str::FromStr;

use super::scrambler::Scrambler;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    SwapPos(usize, usize),
    SwapLett(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    ReverseThrough(usize, usize),
    MovePos(usize, usize),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Instruction, ()> {
        use self::Instruction::*;
        let words = s.split_whitespace().collect::<Vec<_>>();

        if s.starts_with("swap position") {
            let x = words[2].parse().unwrap();
            let y = words[5].parse().unwrap();

            return Ok(SwapPos(x, y));
        }

        if s.starts_with("swap letter") {
            let x = words[2].chars().next().unwrap();
            let y = words[5].chars().next().unwrap();

            return Ok(SwapLett(x, y));
        }

        if s.starts_with("rotate left") {
            let x = words[2].parse().unwrap();
            return Ok(RotateLeft(x));
        }

        if s.starts_with("rotate right") {
            let x = words[2].parse().unwrap();
            return Ok(RotateRight(x));
        }

        if s.starts_with("rotate based") {
            let x = words[6].chars().next().unwrap();
            return Ok(RotateLetter(x));
        }

        if s.starts_with("reverse positions") {
            let x = words[2].parse().unwrap();
            let y = words[4].parse().unwrap();
            return Ok(ReverseThrough(x, y));
        }

        if s.starts_with("move position") {
            let x = words[2].parse().unwrap();
            let y = words[5].parse().unwrap();
            return Ok(MovePos(x, y));
        }

        Err(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InstructionRunner {
    instrs: Vec<Instruction>,
}

impl InstructionRunner {
    pub fn new(instructions: &str) -> Self {
        InstructionRunner {
            instrs: instructions.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    pub fn run<T: ::std::fmt::Debug + Scrambler>(&self, password: &str) -> String {
        let mut s = T::new(password);

        for instr in self.instrs.iter() {
            use self::Instruction::*;
            
            match *instr {
                SwapPos(x, y) => s.swap_pos(x, y),
                SwapLett(x, y) => s.swap_lett(x, y),
                RotateLeft(n) => s.rotate_left(n),
                RotateRight(n) => s.rotate_right(n),
                RotateLetter(x) => s.rotate_letter(x),
                ReverseThrough(x, y) => s.reverse_through(x, y),
                MovePos(x, y) => s.move_pos(x, y),
            }
        }


        s.get_password()
    }
}
