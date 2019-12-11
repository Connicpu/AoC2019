pub fn parse(input: &str) -> Vec<i64> {
    let mut state = State::new();
    for &b in input.as_bytes() {
        state.next(b)
    }
    state.finish()
}

struct State {
    result: Vec<i64>,
    value: i64,
    sign: i64,
    span: usize,
}

impl State {
    fn new() -> State {
        State {
            result: Vec::with_capacity(256),
            value: 0,
            sign: 1,
            span: 0,
        }
    }

    fn next(&mut self, digit: u8) {
        match digit {
            b'-' => self.negative(),
            b'0'..=b'9' => self.push(digit),
            b',' | b'\n' => self.commit(),
            _ => (),
        }
    }

    fn negative(&mut self) {
        self.sign = -1;
        self.span += 1;
    }

    fn push(&mut self, digit: u8) {
        self.value *= 10;
        self.value += (digit - b'0') as i64;
        self.span += 1;
    }

    fn commit(&mut self) {
        if self.span == 0 {
            return;
        }
        self.result.push(self.value * self.sign);
        self.value = 0;
        self.sign = 1;
        self.span = 0;
    }

    fn finish(mut self) -> Vec<i64> {
        self.commit();
        self.result
    }
}
