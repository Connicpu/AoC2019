use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};

pub struct Cpu<I: IO = ()> {
    pub memory: Vec<i64>,
    pub pc: usize,
    pub rbo: i64,
    pub io: I,
}

pub fn parse(input: &str) -> Vec<i64> {
    let mut memory = Vec::with_capacity(256);
    let mut state = 0;
    let mut sign = 1;
    for &b in input.as_bytes() {
        match b {
            b'-' => sign = -1,
            b'0'..=b'9' => state = state * 10 + (b - b'0') as i64,
            b',' | b'\n' => {
                memory.push(state * sign);
                state = 0;
                sign = 1;
            }
            _ => continue,
        }
    }
    memory
}

impl Cpu<()> {
    pub fn new(memory: Vec<i64>) -> Self {
        Cpu::with_io(memory, ())
    }
}

impl<I: IO> Cpu<I> {
    pub fn parse_with_io(input: &str, io: I) -> Self {
        let memory = parse(input);
        Cpu::with_io(memory, io)
    }

    pub fn with_io(memory: Vec<i64>, io: I) -> Self {
        Cpu { memory, pc: 0, rbo: 0, io }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.pc];
            let instruction = opcode % 100;
            let mode1 = (opcode / 1_00) % 10;
            let mode2 = (opcode / 10_00) % 10;
            let mode3 = (opcode / 100_00) % 10;

            match instruction {
                // Add [addr][addr][addr]
                1 => {
                    let a = self.param_get(mode1, 1);
                    let b = self.param_get(mode2, 2);
                    self.param_set(mode3, 3, a + b);
                    self.pc += 4;
                }

                // Mul [addr][addr][addr]
                2 => {
                    let a = self.param_get(mode1, 1);
                    let b = self.param_get(mode2, 2);
                    self.param_set(mode3, 3, a * b);
                    self.pc += 4;
                }

                // Save input to address
                3 => {
                    let a = self.io.input();
                    self.param_set(mode1, 1, a);
                    self.pc += 2;
                }

                // Output from address
                4 => {
                    let a = self.param_get(mode1, 1);
                    self.io.output(a);
                    self.pc += 2;
                }

                // Jump-if-true
                5 => {
                    if self.param_get(mode1, 1) != 0 {
                        self.pc = self.param_get(mode2, 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                // Jump-if-false
                6 => {
                    if self.param_get(mode1, 1) == 0 {
                        self.pc = self.param_get(mode2, 2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }

                // Less than
                7 => {
                    let cond = self.param_get(mode1, 1) < self.param_get(mode2, 2);
                    let value = cond as i64;
                    self.param_set(mode3, 3, value);
                    self.pc += 4;
                }

                // Equals
                8 => {
                    let cond = self.param_get(mode1, 1) == self.param_get(mode2, 2);
                    let value = cond as i64;
                    self.param_set(mode3, 3, value);
                    self.pc += 4;
                }

                // Add RBO
                9 => {
                    let value = self.param_get(mode1, 1);
                    self.rbo += value;
                    self.pc += 2;
                }

                // Halt
                99 => break,
                unk => panic!("Unknown opcode {}", unk),
            }
        }
    }

    fn param_get(&self, mode: i64, offset: usize) -> i64 {
        let addr = self.param_addr(mode, offset);
        if addr >= self.memory.len() {
            assert!(addr < std::isize::MAX as usize, "overflow");
            0
        } else {
            self.memory[addr]
        }
    }

    fn param_set(&mut self, mode: i64, offset: usize, value: i64) {
        let addr = self.param_addr(Self::set_mode(mode), offset);
        if addr >= self.memory.len() {
            assert!(addr < std::isize::MAX as usize, "overflow");
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = value;
    }

    fn set_mode(mode: i64) -> i64 {
        match mode {
            0 => 0,
            1 => 0,
            2 => 2,
            _ => unimplemented!(),
        }
    }

    fn param_addr(&self, mode: i64, offset: usize) -> usize {
        match mode {
            0 => self.memory[self.pc + offset] as usize,
            1 => self.pc + offset,
            2 => (self.rbo + self.memory[self.pc + offset]) as usize,
            _ => unimplemented!(),
        }
    }


}

pub trait IO {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);
}

impl IO for () {
    fn input(&mut self) -> i64 {
        unimplemented!()
    }
    fn output(&mut self, _value: i64) {
        unimplemented!()
    }
}

pub struct StdIO;

impl IO for StdIO {
    fn input(&mut self) -> i64 {
        loop {
            print!("Please enter a number: ");
            std::io::stdout().flush().expect("um");
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("um");
            match line.trim().parse() {
                Ok(i) => return i,
                Err(_) => println!("❌"),
            }
        }
    }
    fn output(&mut self, value: i64) {
        println!("Output: {}", value);
    }
}

pub struct SingleIO {
    pub input: i64,
    pub output: i64,
}

impl SingleIO {
    pub fn new(input: i64) -> SingleIO {
        SingleIO { input, output: 0 }
    }
}

impl IO for SingleIO {
    fn input(&mut self) -> i64 {
        self.input
    }
    fn output(&mut self, value: i64) {
        self.output = value;
    }
}

pub struct ChannelIO {
    input: Receiver<i64>,
    output: Sender<i64>,
    pub last_output: i64,
}

impl ChannelIO {
    pub fn new(input: Receiver<i64>, output: Sender<i64>) -> Self {
        ChannelIO {
            input,
            output,
            last_output: 0,
        }
    }
}

impl IO for ChannelIO {
    fn input(&mut self) -> i64 {
        self.input.recv().unwrap_or(0)
    }
    fn output(&mut self, value: i64) {
        self.last_output = value;
        self.output.send(value).ok();
    }
}
