use num::PrimInt;

pub fn parse<T: PrimInt>(data: &[u8]) -> ParseIter<T> {
    ParseIter {
        state: State::new(),
        data: data.iter(),
    }
}

pub fn parse_i64_vec(data: &str) -> Vec<i64> {
    parse(data.as_bytes()).collect()
}

pub struct ParseIter<'a, T: PrimInt> {
    state: State<T>,
    data: std::slice::Iter<'a, u8>,
}

impl<T: PrimInt> Iterator for ParseIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        for &b in self.data.by_ref() {
            if let Some(next) = self.state.next(b) {
                return Some(next);
            }
        }
        self.state.commit()
    }
}

struct State<T: PrimInt> {
    value: T,
    sign: T,
    span: usize,
}

impl<T: PrimInt> State<T> {
    fn new() -> State<T> {
        State {
            value: T::zero(),
            sign: T::one(),
            span: 0,
        }
    }

    fn next(&mut self, digit: u8) -> Option<T> {
        match digit {
            b'-' => self.negative(),
            b'0'..=b'9' => self.push(digit),
            b',' | b'\n' => return self.commit(),
            _ => (),
        }

        None
    }

    fn negative(&mut self) {
        self.sign = !T::zero();
        self.span += 1;
    }

    fn push(&mut self, digit: u8) {
        self.value = self.value * T::from(10).unwrap();
        self.value = self.value + T::from(digit - b'0').unwrap();
        self.span += 1;
    }

    fn commit(&mut self) -> Option<T> {
        if self.span == 0 {
            return None;
        }
        let res = self.value * self.sign;
        self.value = T::zero();
        self.sign = T::one();
        self.span = 0;
        Some(res)
    }
}
