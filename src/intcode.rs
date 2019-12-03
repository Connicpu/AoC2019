pub struct Cpu {
    pub memory: Vec<i32>,
    pub pc: usize,
}

impl Cpu {
    pub fn new(memory: Vec<i32>) -> Cpu {
        Cpu {
            memory,
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.memory[self.pc] {
                // Add [addr][addr][addr]
                1 => {
                    let a = self.pcr_get(1);
                    let b = self.pcr_get(2);
                    self.pcr_set(3, a + b);
                    self.pc += 4;
                }

                // Mul [addr][addr][addr]
                2 => {
                    let a = self.pcr_get(1);
                    let b = self.pcr_get(2);
                    self.pcr_set(3, a * b);
                    self.pc += 4;
                }

                // Halt
                99 => break,
    
                _ => panic!(),
            }
        }
    }

    fn pcr_get(&self, offset: usize) -> i32 {
        let addr = self.memory[self.pc + offset] as usize;
        self.memory[addr]
    }

    fn pcr_set(&mut self, offset: usize, value: i32) {
        let addr = self.memory[self.pc + offset] as usize;
        self.memory[addr] = value;
    }
}