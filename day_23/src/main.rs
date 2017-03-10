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

impl Register {
    pub fn is_reg(&self) -> bool {
        use Register::*;
        match *self {
            A | B | C | D => true,
            _ => false
        }
    }

    pub fn is_lit(&self) -> bool {
        use Register::*;
        match *self {
            A | B | C |D => false,
            _ => true
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Cpy(Register, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, Register),
    Tgl(Register),
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
            "tgl" => Instruction::Tgl(parts[1].parse().unwrap()),
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
                    self.ptr = (self.ptr as isize + self.get_reg(offset)) as usize;
                } else {
                    self.ptr += 1;
                }
            },
            Tgl(offset) => {
                let idx = (self.get_reg(offset) + (self.ptr as isize)) as usize;
                if idx >= self.instructions.len() {
                    self.ptr += 1;
                    return;
                }
                let target_instr = self.instructions[idx];
                
                let new_instr = match target_instr {
                    Inc(x) => Dec(x),
                    Dec(x) => Inc(x),
                    Tgl(x) => {
                        if x.is_reg() {
                            Inc(x)
                        } else {
                            Tgl(x)
                        }
                    },
                    Jnz(x, y) => {
                        if y.is_reg() {
                            Cpy(x, y)
                        } else {
                            Jnz(x, y)
                        }
                    },
                    Cpy(x, y) => {
                        Jnz(x, y)
                    }
                };

                self.instructions[idx] = new_instr;
                self.ptr += 1;
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

const TEST_INPUT: &'static str = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";

const PUZZLE_INPUT: &'static str = "cpy a b
dec b
cpy a d
cpy 0 a
cpy b c
inc a
dec c
jnz c -2
dec d
jnz d -5
dec b
cpy b c
cpy c d
dec d
inc c
jnz d -2
tgl c
cpy -16 c
jnz 1 c
cpy 95 c
jnz 73 d
inc a
inc d
jnz d -2
inc c
jnz c -5";

fn main() {
    let mut cpu = Cpu::new(TEST_INPUT.lines().map(Instruction::from_str).collect());
    cpu.run();
    println!("Test machine output: {}", cpu.a);

    cpu = Cpu::new(PUZZLE_INPUT.lines().map(Instruction::from_str).collect());
    cpu.a = 7;
    cpu.run();
    println!("Challenge machine output: {}", cpu.a);

    cpu = Cpu::new(PUZZLE_INPUT.lines().map(Instruction::from_str).collect());
    cpu.a = 12;
    cpu.run();
    println!("Challenge machine output part 2: {}", cpu.a);
}
