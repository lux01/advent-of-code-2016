//! # Advent of Code 2016 - Day 1
//!
//! This is my Rust solution to Day 1 of the Advent of Code 2016
//! "No Time for a Taxicab." Full details of the challenge can be
//! found on [the challenge page][page].
//!
//! [page]: http://adventofcode.com/2016/day/1

use std::collections::HashMap;
use std::str::FromStr;

/// The compass direction Santa's little helper is facing.
#[derive(Debug)]
pub enum Orientation {
    North,
    West,
    South,
    East,
}

impl Orientation {
    /// Returns the new orientation after turning left.
    pub fn left(self) -> Orientation {
        use Orientation::*;
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }

    /// Returns the new orientation after turning right.
    pub fn right(self) -> Orientation {
        use Orientation::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

/// The position and orientation of Santa's little helper in the city grid.
#[derive(Debug)]
pub struct Position {
    /// The East-West direction with East being positive.
    pub x: i64,
    /// The North-South direction with North being positive.
    pub y: i64,
    /// The direction Santa's little helper is facing.
    pub facing: Orientation,
    
}

impl Position {
    /// Puts a new Santa's little helper in the grid at position
    /// `(0, 0)` facing North.
    pub fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            facing: Orientation::North,
        }
    }


    /// Follows one step of the Easter Bunny's instructions
    pub fn follow_instruction(self, instruction: Instruction) -> Position {
        self.turn(instruction.direction)
            .walk(instruction.amount)
    }
    
    fn walk(self, amount: i64) -> Position {
        use Orientation::*;

        let (new_x, new_y) = match self.facing {
            North => (self.x, self.y + amount),
            West => (self.x - amount, self.y),
            South => (self.x, self.y - amount),
            East => (self.x + amount, self.y),
        };

        Position {
            x: new_x,
            y: new_y,
            ..self
        }
    }

    fn turn(self, direction: TurnDirection) -> Position {
        match direction {
            TurnDirection::Left  => self.left(),
            TurnDirection::Right => self.right(),
        }
    }
    
    fn left(self) -> Position {
        Position { facing: self.facing.left(), ..self }
    }

    fn right(self) -> Position {
        Position { facing: self.facing.right(), ..self }
    }

    /// Calculates how many blocks away from the start that
    /// Santa's little helper has travelled.
    pub fn blocks_travelled(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

/// A direction to turn in from the Easter Bunny's instructions.
#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

/// One of the Easter Bunny's instructions on how to find his HQ.
#[derive(Debug)]
pub struct Instruction {
    amount: i64,
    direction: TurnDirection,
}

/// A parse error interpreting an Easter Bunny instruction.
#[derive(Debug)]
pub enum InstructionParseErr {
    /// The direction was not known.
    BadDirection,
    /// The move amount could not be parsed.
    BadAmount(std::num::ParseIntError),
    /// The input was malformed.
    BadInput,
}

impl FromStr for Instruction {
    type Err = InstructionParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let dir = match chars.next() {
            Some('R') => Ok(TurnDirection::Right),
            Some('L') => Ok(TurnDirection::Left),
            Some(_)   => Err(InstructionParseErr::BadDirection),
            None      => Err(InstructionParseErr::BadInput),
        }?;

        let amount = chars.as_str()
            .parse::<i64>()
            .map_err(InstructionParseErr::BadAmount)?;

        Ok(Instruction {
            amount: amount,
            direction: dir,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input_str = "R3, L2";
        let destination = process_input(input_str);

        assert!(destination.is_ok());
        assert_eq!(destination.unwrap().blocks_travelled(), 5);
    }

    #[test]
    fn test_input_2() {
        let input_str = "R2, R2, R2";
        let destination = process_input(input_str);

        assert!(destination.is_ok());
        assert_eq!(destination.unwrap().blocks_travelled(), 2);
    }

    #[test]
    fn test_input_3() {
        let input_str = "R5, L5, R5, R3";
        let destination = process_input(input_str);

        assert!(destination.is_ok());
        assert_eq!(destination.unwrap().blocks_travelled(), 12);
    }
}

/// Given an input string, calculates the final destination.
pub fn process_input(input_str: &str) -> Result<Position, InstructionParseErr> {
    input_str.split(", ")
        .map(|s| s.parse::<Instruction>())
        .fold(Ok(Position::new()),
              |pos_opt, instr_opt| {
                  match (pos_opt, instr_opt) {
                      (Ok(pos), Ok(instr)) => Ok(pos.follow_instruction(instr)),
                      (Err(a), _) => Err(a),
                      (_, Err(b)) => Err(b),
                  }
              })
}

/// Calculates the location of the Easter Bunny's HQ, which is the first
/// intersection visited twice when following the instructions.
pub fn find_bunny_hq(input_str: &str) -> Result<(i64, i64), InstructionParseErr> {
    let instructions = input_str.split(", ")
        .map(|s| s.parse::<Instruction>())
        .collect::<Result<Vec<Instruction>, InstructionParseErr>>()?;

    // Construct a map to hold all the locations that we have visited
    let mut visited = HashMap::new();
    let mut posn = Position::new();
    visited.insert((0, 0), 1);

    for instr in instructions {
        posn = posn.turn(instr.direction);
        for _ in 1 .. (instr.amount + 1) {
            posn = posn.walk(1);
            if visited.get(&(posn.x, posn.y)).is_some() {
                return Ok((posn.x, posn.y))
            } else {
                visited.insert((posn.x, posn.y), 1);
            }
            
        }
    }

    Ok((std::i64::MAX, std::i64::MAX))
}


fn main() {
    let challenge_input = "R2, L1, R2, R1, R1, L3, R3, L5, L5, L2, L1, R4, R1, R3, L5, L5, R3, L4, L4, R5, R4, R3, L1, L2, R5, R4, L2, R1, R4, R4, L2, L1, L1, R190, R3, L4, R52, R5, R3, L5, R3, R2, R1, L5, L5, L4, R2, L3, R3, L1, L3, R5, L3, L4, R3, R77, R3, L2, R189, R4, R2, L2, R2, L1, R5, R4, R4, R2, L2, L2, L5, L1, R1, R2, L3, L4, L5, R1, L1, L2, L2, R2, L3, R3, L4, L1, L5, L4, L4, R3, R5, L2, R4, R5, R3, L2, L2, L4, L2, R2, L5, L4, R3, R1, L2, R2, R4, L1, L4, L4, L2, R2, L4, L1, L1, R4, L1, L3, L2, L2, L5, R5, R2, R5, L1, L5, R2, R4, R4, L2, R5, L5, R5, R5, L4, R2, R1, R1, R3, L3, L3, L4, L3, L2, L2, L2, R2, L1, L3, R2, R5, R5, L4, R3, L3, L4, R2, L5, R5";

    let destination = process_input(challenge_input).expect("Failed to process instructions");
    let bunny_hq = find_bunny_hq(challenge_input).expect("Failed to process instructions");
    
    println!("Final location = ({}, {}), facing {:?}",
             destination.x,
             destination.y,
             destination.facing);
    println!("Distance from the start = {}\n", destination.blocks_travelled());
    println!("First place visited twice = ({}, {})", bunny_hq.0, bunny_hq.1);
    println!("Distance from the start = {}", bunny_hq.0.abs() + bunny_hq.1.abs());
}
