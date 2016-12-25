mod scrambler;
mod instruction;
mod pass_scrambler;
mod pass_unscrambler;

use instruction::InstructionRunner;
use pass_scrambler::PassScrambler;
use pass_unscrambler::PassUnscrambler;

macro_rules! read_file {
    ($fname:expr) => {{
        use std::fs::File;
        use std::io::Read;
        
        let mut buff = String::new();
        let mut f = File::open($fname).unwrap();
        f.read_to_string(&mut buff).unwrap();
        buff
    }}
}

fn main() {
    let test_input = read_file!("test_input.txt");
    let input = read_file!("input.txt");

    let mut input_rev = input.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
    input_rev.reverse();
    let input_rev = input_rev.into_iter()
        .fold("".to_owned(), |acc, s| if acc.len() > 0 { format!("{}\n{}", acc, s) } else { s });

    
    let test_runner = InstructionRunner::new(&test_input);
    let full_runner = InstructionRunner::new(&input);
    let back_runner = InstructionRunner::new(&input_rev);

    let test_pass = test_runner.run::<PassScrambler>("abcde");
    let part1 = full_runner.run::<PassScrambler>("abcdefgh");
    let part2_test = back_runner.run::<PassUnscrambler>(&part1);
    let part2_full = back_runner.run::<PassUnscrambler>("fbgdceah");
    
    println!("Test case: abcde => {}", test_pass);
    println!("Part 1: abcdefgh => {}", part1);
    println!("Part 2: {} => {}", part1, part2_test);
    println!("Part 2: fbgdceah => {}", part2_full);

}
