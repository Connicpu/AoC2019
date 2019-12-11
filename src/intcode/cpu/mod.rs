use super::parse;
use super::IO;

mod instructions;
mod addressing;

pub struct Cpu<T: IO = ()> {
    pub memory: Vec<i64>,
    pub pc: usize,
    pub rbo: i64,
    pub io: T,
}

impl Cpu<()> {
    pub fn new(memory: Vec<i64>) -> Self {
        Cpu::with_io(memory, ())
    }
}

impl<T: IO> Cpu<T> {
    pub fn parse_with_io(input: &str, io: T) -> Self {
        let memory = parse(input);
        Cpu::with_io(memory, io)
    }

    pub fn with_io(memory: Vec<i64>, io: T) -> Self {
        Cpu {
            memory,
            pc: 0,
            rbo: 0,
            io,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.memory[self.pc] % 100 {
                1 => self.i_add(),
                2 => self.i_mul(),
                3 => self.i_in(),
                4 => self.i_out(),
                5 => self.i_jnz(),
                6 => self.i_jz(),
                7 => self.i_lt(),
                8 => self.i_eq(),
                9 => self.i_arbo(),

                // Halt
                99 => break,
                op => panic!("Unknown opcode {}", op),
            }
        }
    }
}
