use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};

pub struct Cpu<I: IO = ()> {
    pub memory: Vec<i32>,
    pub pc: usize,
    pub io: I,
}

pub fn parse(input: &str) -> Vec<i32> {
    let mut memory = Vec::with_capacity(256);
    let mut state = 0;
    let mut sign = 1;
    for &b in input.as_bytes() {
        match b {
            b'-' => sign = -1,
            b'0'..=b'9' => state = state * 10 + (b - b'0') as i32,
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
    pub fn new(memory: Vec<i32>) -> Self {
        Cpu::with_io(memory, ())
    }
}

impl<I: IO> Cpu<I> {
    pub fn parse_with_io(input: &str, io: I) -> Self {
        let memory = parse(input);
        Cpu::with_io(memory, io)
    }

    pub fn with_io(memory: Vec<i32>, io: I) -> Self {
        Cpu { memory, pc: 0, io }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.pc];
            let instruction = opcode % 100;
            let mode1 = (opcode / 1_00) % 10;
            let mode2 = (opcode / 10_00) % 10;

            match instruction {
                // Add [addr][addr][addr]
                1 => {
                    let a = self.param_get(mode1, 1);
                    let b = self.param_get(mode2, 2);
                    self.param_set(3, a + b);
                    self.pc += 4;
                }

                // Mul [addr][addr][addr]
                2 => {
                    let a = self.param_get(mode1, 1);
                    let b = self.param_get(mode2, 2);
                    self.param_set(3, a * b);
                    self.pc += 4;
                }

                // Save input to address
                3 => {
                    let a = self.io.input();
                    self.param_set(1, a);
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
                    let value = cond as i32;
                    self.param_set(3, value);
                    self.pc += 4;
                }

                // Equals
                8 => {
                    let cond = self.param_get(mode1, 1) == self.param_get(mode2, 2);
                    let value = cond as i32;
                    self.param_set(3, value);
                    self.pc += 4;
                }

                // Halt
                99 => break,
                unk => panic!("Unknown opcode {}", unk),
            }
        }
    }

    fn param_get(&self, mode: i32, offset: usize) -> i32 {
        let addr = self.param_addr(mode, offset);
        self.memory[addr]
    }

    fn param_set(&mut self, offset: usize, value: i32) {
        let addr = self.param_addr(0, offset);
        self.memory[addr] = value;
    }

    fn param_addr(&self, mode: i32, offset: usize) -> usize {
        match mode {
            0 => self.memory[self.pc + offset] as usize,
            1 => self.pc + offset,
            _ => unimplemented!(),
        }
    }
}

pub trait IO {
    fn input(&mut self) -> i32;
    fn output(&mut self, value: i32);
}

impl IO for () {
    fn input(&mut self) -> i32 {
        unimplemented!()
    }
    fn output(&mut self, _value: i32) {
        unimplemented!()
    }
}

pub struct StdIO;

impl IO for StdIO {
    fn input(&mut self) -> i32 {
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
    fn output(&mut self, value: i32) {
        println!("Output: {}", value);
    }
}

pub struct SingleIO {
    pub input: i32,
    pub output: i32,
}

impl SingleIO {
    pub fn new(input: i32) -> SingleIO {
        SingleIO { input, output: 0 }
    }
}

impl IO for SingleIO {
    fn input(&mut self) -> i32 {
        self.input
    }
    fn output(&mut self, value: i32) {
        self.output = value;
    }
}

pub struct OnceIO {
    pub input: i32,
    pub output: i32,
}

impl OnceIO {
    pub fn new(input: i32) -> OnceIO {
        OnceIO { input, output: 0 }
    }
}

impl IO for OnceIO {
    fn input(&mut self) -> i32 {
        let val = self.input;
        self.input = 0;
        val
    }
    fn output(&mut self, value: i32) {
        self.output = value;
    }
}

pub struct IterIO<I: Iterator<Item = i32>> {
    iter: I,
    pub output: i32,
}

impl<I: Iterator<Item = i32>> IterIO<I> {
    pub fn new(iter: I) -> IterIO<I> {
        IterIO { iter, output: 0 }
    }
}

impl<I: Iterator<Item = i32>> IO for IterIO<I> {
    fn input(&mut self) -> i32 {
        self.iter.next().unwrap()
    }
    fn output(&mut self, value: i32) {
        self.output = value;
    }
}

pub struct LoopIO {
    pub input: Vec<i32>,
    pub index: usize,
    pub output: i32,
}

impl LoopIO {
    pub fn new(input: Vec<i32>) -> LoopIO {
        LoopIO {
            input,
            index: 0,
            output: 0,
        }
    }
}

impl IO for LoopIO {
    fn input(&mut self) -> i32 {
        let val = self.input[self.index];
        self.index = (self.index + 1) % self.input.len();
        val
    }
    fn output(&mut self, value: i32) {
        self.output = value;
    }
}

pub struct ChannelIO {
    input: Receiver<i32>,
    output: Sender<i32>,
    pub last_output: i32,
}

impl ChannelIO {
    pub fn new(input: Receiver<i32>, output: Sender<i32>) -> Self {
        ChannelIO {
            input,
            output,
            last_output: 0,
        }
    }
}

impl IO for ChannelIO {
    fn input(&mut self) -> i32 {
        self.input.recv().unwrap_or(0)
    }
    fn output(&mut self, value: i32) {
        self.last_output = value;
        self.output.send(value).ok();
    }
}
