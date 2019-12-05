use aoc2019::intcode::{Cpu, StdIO};

static INPUT: &str = include_str!("input/day05.txt");

fn main() {
    let mut cpu = Cpu::parse_with_io(INPUT, StdIO);
    cpu.run();
}
