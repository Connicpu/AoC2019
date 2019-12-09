use aoc2019::intcode::{parse, SingleIO, Cpu};

use once_cell::sync::Lazy;

static INPUT: &str = include_str!("input/day09.txt");
static PROGRAM: Lazy<Vec<i64>> = Lazy::new(|| parse(INPUT));

fn compute_boost(input: i64) -> i64 {
    let mut cpu = Cpu::with_io(PROGRAM.clone(), SingleIO::new(input));
    cpu.run();
    cpu.io.output
}

fn main() {
    println!("Part 1: {}", compute_boost(1));
    println!("Part 2: {}", compute_boost(2));
}
