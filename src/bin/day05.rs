#![cfg_attr(test, feature(test))]
#[cfg(test)] extern crate test;

use aoc2019::intcode::{Cpu, SingleIO};

static INPUT: &str = include_str!("input/day05.txt");

const PART_1_INPUT: i32 = 1;
const PART_2_INPUT: i32 = 5;

fn calculate(input: i32) -> i32 {
    let mut cpu = Cpu::parse_with_io(INPUT, SingleIO::new(input));
    cpu.run();
    cpu.io.output
}

fn main() {
    println!("Part 1: {}", calculate(PART_1_INPUT));
    println!("Part 2: {}", calculate(PART_2_INPUT));
}

#[cfg(test)]
#[bench]
fn part_1_benchmark(bench: &mut test::Bencher) {
    bench.iter(|| {
        test::black_box(calculate(PART_1_INPUT));
    });
}

#[cfg(test)]
#[bench]
fn part_2_benchmark(bench: &mut test::Bencher) {
    bench.iter(|| {
        test::black_box(calculate(PART_2_INPUT));
    });
}

#[cfg(test)]
#[bench]
fn part_1_benchmark_excl_parse(bench: &mut test::Bencher) {
    let memory = aoc2019::intcode::parse(INPUT);
    bench.iter(|| {
        let mut cpu = Cpu::with_io(memory.clone(), SingleIO::new(PART_1_INPUT));
        cpu.run();
        test::black_box(cpu.io.output);
    });
}

#[cfg(test)]
#[bench]
fn part_2_benchmark_excl_parse(bench: &mut test::Bencher) {
    let memory = aoc2019::intcode::parse(INPUT);
    bench.iter(|| {
        let mut cpu = Cpu::with_io(memory.clone(), SingleIO::new(PART_2_INPUT));
        cpu.run();
        test::black_box(cpu.io.output);
    });
}
