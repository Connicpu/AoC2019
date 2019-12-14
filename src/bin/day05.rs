#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2019::intcode::Cpu;

static INPUT: &str = include_str!("input/day05.txt");

const PART_1_INPUT: i64 = 1;
const PART_2_INPUT: i64 = 5;

fn calculate(input: i64) -> i64 {
    Cpu::parse(INPUT).compute(input)
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
        let mut cpu = Cpu::new(memory.clone());
        test::black_box(cpu.compute(PART_1_INPUT));
    });
}

#[cfg(test)]
#[bench]
fn part_2_benchmark_excl_parse(bench: &mut test::Bencher) {
    let memory = aoc2019::intcode::parse(INPUT);
    bench.iter(|| {
        let mut cpu = Cpu::new(memory.clone());
        test::black_box(cpu.compute(PART_2_INPUT));
    });
}
