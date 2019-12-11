use super::Cpu;
use super::IO;

impl<T: IO> Cpu<T> {
    pub(super) fn arg_get(&self, arg: usize) -> i64 {
        let addr = self.arg_addr(false, arg);
        if addr >= self.memory.len() {
            assert!(addr < std::isize::MAX as usize, "overflow");
            0
        } else {
            self.memory[addr]
        }
    }

    pub(super) fn arg_set(&mut self, arg: usize, value: i64) {
        let addr = self.arg_addr(true, arg);
        if addr >= self.memory.len() {
            assert!(addr < std::isize::MAX as usize, "overflow");
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = value;
    }

    pub(super) fn arg_addr(&self, store: bool, arg: usize) -> usize {
        let instr = self.memory[self.pc];
        let mode = (instr / 10i64.pow(arg as u32 + 1)) % 10;
        match mode & !(store as i64) {
            0 => self.memory[self.pc + arg] as usize,
            1 => self.pc + arg,
            2 => (self.rbo + self.memory[self.pc + arg]) as usize,
            _ => panic!("Unknown addressing mode {}", mode),
        }
    }
}
