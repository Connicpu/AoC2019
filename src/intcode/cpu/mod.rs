use super::parse;
use super::IO;
use super::SingleIO;

mod addressing;
mod instructions;

pub struct Cpu {
    pub memory: Vec<i64>,
    pub pc: usize,
    pub rbo: i64,
}

impl Cpu {
    pub fn new(memory: Vec<i64>) -> Self {
        Cpu {
            memory,
            pc: 0,
            rbo: 0,
        }
    }
}

impl Cpu {
    pub fn parse(input: &str) -> Self {
        let memory = parse(input);
        Cpu::new(memory)
    }

    pub fn run(&mut self, mut io: impl IO) {
        loop {
            match self.resume() {
                CpuResult::Halt => break,
                CpuResult::Input => self.input(io.input()),
                CpuResult::Output(out) => io.output(out),
            }
        }
    }

    pub fn resume(&mut self) -> CpuResult {
        loop {
            match self.memory[self.pc] % 100 {
                1 => self.i_add(),
                2 => self.i_mul(),
                3 => break CpuResult::Input,
                4 => break CpuResult::Output(self.i_out()),
                5 => self.i_jnz(),
                6 => self.i_jz(),
                7 => self.i_lt(),
                8 => self.i_eq(),
                9 => self.i_arbo(),

                // Halt
                99 => break CpuResult::Halt,
                op => panic!("Unknown opcode {}", op),
            }
        }
    }

    pub fn input(&mut self, input: i64) {
        self.i_in(input);
    }

    pub fn compute(&mut self, input: i64) -> i64 {
        let mut io = SingleIO::new(input);
        self.run(&mut io);
        io.output
    }
}

pub enum CpuResult {
    Halt,
    Input,
    Output(i64),
}
