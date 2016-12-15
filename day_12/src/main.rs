use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    Literal(isize),
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _   => s.parse::<isize>()
                .map(Register::Literal)
                .map_err(|_| ()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Cpy(Register, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, isize),
}

impl Instruction {
    pub fn from_str(s: &str) -> Instruction {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        match parts[0] {
            "cpy" => Instruction::Cpy(parts[1].parse().unwrap(),
                                      parts[2].parse().unwrap()),
            "inc" => Instruction::Inc(parts[1].parse().unwrap()),
            "dec" => Instruction::Dec(parts[1].parse().unwrap()),
            "jnz" => Instruction::Jnz(parts[1].parse().unwrap(),
                                      parts[2].parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Cpu {
    pub a: isize,
    pub b: isize,
    pub c: isize,
    pub d: isize,
    pub ptr: usize,
    pub instructions: Vec<Instruction>,
}

impl Cpu {
    pub fn new(instrs: Vec<Instruction>) -> Cpu {
        Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            ptr: 0,
            instructions: instrs,
        }
    }

    fn get_reg(&self, reg: Register) -> isize {
        use Register::*;
        match reg {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            Literal(x) => x,
        }
    }

    fn set_reg(&mut self, reg: Register, val: isize) {
        match reg {
            Register::A => self.a = val,
            Register::B => self.b = val,
            Register::C => self.c = val,
            Register::D => self.d = val,
            _ => ()
        }
    }

    fn update_reg<F>(&mut self, reg: Register, f: F)
        where F: FnOnce(isize) -> isize
    {
        let old_val = self.get_reg(reg);
        self.set_reg(reg, f(old_val));
    }
    
    fn exec(&mut self, instr: Instruction) {
        use Instruction::*;

        match instr {
            Cpy(src, dst) => {
                let src_val = self.get_reg(src);
                self.set_reg(dst, src_val);
                self.ptr += 1
            },
            Inc(reg) => { self.update_reg(reg, |x| x + 1); self.ptr += 1 },
            Dec(reg) => { self.update_reg(reg, |x| x - 1); self.ptr += 1},
            Jnz(src, offset) => {
                if self.get_reg(src) != 0 {
                    self.ptr = (self.ptr as isize + offset) as usize;
                } else {
                    self.ptr += 1;
                }
            },
        }
    }
    
    pub fn run(&mut self) {
        while self.ptr < self.instructions.len() {
            let instr = self.instructions[self.ptr];
            self.exec(instr);
        }
    }

    
}

const TEST_INPUT: &'static str = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a";

const PUZZLE_INPUT: &'static str = "cpy 1 a
cpy 1 b
cpy 26 d
jnz c 2
jnz 1 5
cpy 7 c
inc d
dec c
jnz c -2
cpy a c
inc a
dec b
jnz b -2
cpy c b
dec d
jnz d -6
cpy 16 c
cpy 12 d
inc a
dec d
jnz d -2
dec c
jnz c -5";

fn main() {
    let mut cpu = Cpu::new(TEST_INPUT.lines().map(Instruction::from_str).collect());
    cpu.run();
    println!("Test machine output: {}", cpu.a);

    cpu = Cpu::new(PUZZLE_INPUT.lines().map(Instruction::from_str).collect());
    cpu.run();
    println!("Challenge machine output: {}", cpu.a);

    cpu = Cpu::new(PUZZLE_INPUT.lines().map(Instruction::from_str).collect());
    cpu.c = 1;
    cpu.run();
    println!("Challenge machine output part 2: {}", cpu.a);
}
