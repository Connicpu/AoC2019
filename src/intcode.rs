use std::io::Write;

pub struct Cpu<I: IO = ()> {
    pub memory: Vec<i32>,
    pub pc: usize,
    pub io: I,
}

impl Cpu<()> {
    pub fn new(memory: Vec<i32>) -> Self {
        Cpu::with_io(memory, ())
    }
}

impl<I: IO> Cpu<I> {
    pub fn parse_with_io(input: &str, io: I) -> Self {
        let memory = input
            .split(',')
            .filter_map(|i| i.trim().parse().ok())
            .collect();
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

                // equals
                8 => {
                    let cond = self.param_get(mode1, 1) == self.param_get(mode2, 2);
                    let value = cond as i32;
                    self.param_set(3, value);
                    self.pc += 4;
                }

                // Halt
                99 => break,
                _ => panic!(),
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
