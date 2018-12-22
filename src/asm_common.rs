use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Index, IndexMut},
    str::FromStr,
};

use regex::Regex;

lazy_static! {
    static ref RGX: Regex = Regex::new("#ip (\\d+)").unwrap();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl FromStr for Opcode {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, !> {
        let opcode: Self = match s {
            "addr" => Opcode::Addr,
            "addi" => Opcode::Addi,
            "mulr" => Opcode::Mulr,
            "muli" => Opcode::Muli,
            "banr" => Opcode::Banr,
            "bani" => Opcode::Bani,
            "borr" => Opcode::Borr,
            "bori" => Opcode::Bori,
            "setr" => Opcode::Setr,
            "seti" => Opcode::Seti,
            "gtir" => Opcode::Gtir,
            "gtri" => Opcode::Gtri,
            "gtrr" => Opcode::Gtrr,
            "eqir" => Opcode::Eqir,
            "eqri" => Opcode::Eqri,
            "eqrr" => Opcode::Eqrr,
            _ => panic!("Invalid opcode: {}", s),
        };
        Ok(opcode)
    }
}

const REGISTER_COUNT: usize = 6;

pub fn parse_ip_decl(s: &str) -> Option<usize> {
    RGX.captures(s).map(|cap| cap[1].parse().unwrap())
}

#[derive(Clone, Debug)]
pub struct Registers {
    pub ip_register: Option<usize>,
    pub regs: [usize; REGISTER_COUNT],
}

impl Index<usize> for Registers {
    type Output = usize;

    fn index(&self, i: usize) -> &usize { &self.regs[i] }
}

impl IndexMut<usize> for Registers {
    fn index_mut(&mut self, i: usize) -> &mut usize { &mut self.regs[i] }
}

impl Display for Registers {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "[{} {} {} {} {} {}]",
            self.regs[0], self.regs[1], self.regs[2], self.regs[3], self.regs[4], self.regs[5]
        )
    }
}

impl Registers {
    pub fn new(ip_register: Option<usize>) -> Self {
        Registers {
            ip_register,
            regs: [0; REGISTER_COUNT],
        }
    }

    /// Runs a single instruction, mutating the register values and returning the new instruction
    /// pointer.
    pub fn exec(&mut self, ip: usize, instr: Instruction) -> usize {
        if let Some(ip_register) = self.ip_register {
            self.regs[ip_register] = ip;
        }

        self.regs[instr.out] = match instr.opcode {
            Opcode::Addr => self.regs[instr.in1] + self.regs[instr.in2],
            Opcode::Addi => self.regs[instr.in1] + instr.in2,
            Opcode::Mulr => self.regs[instr.in1] * self.regs[instr.in2],
            Opcode::Muli => self.regs[instr.in1] * instr.in2,
            Opcode::Banr => self.regs[instr.in1] & self.regs[instr.in2],
            Opcode::Bani => self.regs[instr.in1] & instr.in2,
            Opcode::Borr => self.regs[instr.in1] | self.regs[instr.in2],
            Opcode::Bori => self.regs[instr.in1] | instr.in2,
            Opcode::Setr => self.regs[instr.in1],
            Opcode::Seti => instr.in1,
            Opcode::Gtir => (instr.in1 > self.regs[instr.in2]) as usize,
            Opcode::Gtri => (self.regs[instr.in1] > instr.in2) as usize,
            Opcode::Gtrr => (self.regs[instr.in1] > self.regs[instr.in2]) as usize,
            Opcode::Eqir => (instr.in1 == self.regs[instr.in2]) as usize,
            Opcode::Eqri => (self.regs[instr.in1] == instr.in2) as usize,
            Opcode::Eqrr => (self.regs[instr.in1] == self.regs[instr.in2]) as usize,
        };

        if let Some(ip_register) = self.ip_register {
            return self.regs[ip_register];
        } else {
            return ip;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub in1: usize,
    pub in2: usize,
    pub out: usize,
}

impl Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "{:?} {} {} {}",
            self.opcode, self.in1, self.in2, self.out
        )
    }
}

#[derive(Clone)]
pub struct Instructions {
    pub ip: usize,
    pub instructions: Vec<Instruction>,
}

impl Index<usize> for Instructions {
    type Output = Instruction;

    fn index(&self, i: usize) -> &Instruction { &self.instructions[i] }
}

impl IndexMut<usize> for Instructions {
    fn index_mut(&mut self, i: usize) -> &mut Instruction { &mut self.instructions[i] }
}

#[derive(Clone)]
pub struct VM {
    pub regs: Registers,
    pub instructions: Instructions,
}

impl VM {
    /// Executes the next instruction in the program.  Handles incrementing the IP and jumping.
    /// Returns `true` if the program has halted and `false` otherwise.
    pub fn tick(&mut self) -> bool {
        let &cur_instr = match self.instructions.instructions.get(self.ip()) {
            Some(instr) => instr,
            None => return true,
        };
        let ip = self.ip();

        self.instructions.ip = self.regs.exec(ip, cur_instr);
        self.instructions.ip += 1;

        false
    }

    /// Runs the VM until it halts.
    pub fn run(&mut self) {
        loop {
            let should_halt = self.tick();
            if should_halt {
                break;
            }
        }
    }

    pub fn ip(&self) -> usize { self.instructions.ip }
}
