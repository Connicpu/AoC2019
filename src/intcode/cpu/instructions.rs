use super::Cpu;

impl Cpu {
    pub(super) fn i_add(&mut self) {
        let a = self.arg_get(1);
        let b = self.arg_get(2);
        self.arg_set(3, a + b);
        self.pc += 4;
    }

    pub(super) fn i_mul(&mut self) {
        let a = self.arg_get(1);
        let b = self.arg_get(2);
        self.arg_set(3, a * b);
        self.pc += 4;
    }

    pub(super) fn i_in(&mut self, input: i64) {
        self.arg_set(1, input);
        self.pc += 2;
    }

    pub(super) fn i_out(&mut self) -> i64 {
        let val = self.arg_get(1);
        self.pc += 2;
        val
    }

    pub(super) fn i_jnz(&mut self) {
        if self.arg_get(1) != 0 {
            self.pc = self.arg_get(2) as usize;
        } else {
            self.pc += 3;
        }
    }

    pub(super) fn i_jz(&mut self) {
        if self.arg_get(1) == 0 {
            self.pc = self.arg_get(2) as usize;
        } else {
            self.pc += 3;
        }
    }

    pub(super) fn i_lt(&mut self) {
        let cond = self.arg_get(1) < self.arg_get(2);
        let value = cond as i64;
        self.arg_set(3, value);
        self.pc += 4;
    }

    pub(super) fn i_eq(&mut self) {
        let cond = self.arg_get(1) == self.arg_get(2);
        let value = cond as i64;
        self.arg_set(3, value);
        self.pc += 4;
    }

    pub(super) fn i_arbo(&mut self) {
        let value = self.arg_get(1);
        self.rbo += value;
        self.pc += 2;
    }
}
