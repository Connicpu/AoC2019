use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};

pub trait IO {
    fn input(&mut self) -> i64;
    fn output(&mut self, value: i64);
}

impl<T: IO> IO for &mut T {
    fn input(&mut self) -> i64 {
        T::input(self)
    }
    fn output(&mut self, value: i64) {
        T::output(self, value)
    }
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
