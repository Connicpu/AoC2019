use aoc2019::intcode::{parse, Cpu, CpuResult};

use once_cell::sync::Lazy;
use permute::permutations_of;

static INPUT: &str = include_str!("input/day07.txt");
static PROGRAM: Lazy<Vec<i64>> = Lazy::new(|| parse(INPUT));

fn try_sequence<'a>(seq: impl Iterator<Item = &'a i64>) -> i64 {
    let mut cpus = Vec::with_capacity(5);

    // Create each CPU and input its phase
    for &phase in seq {
        let mut cpu = Cpu::new(PROGRAM.clone());
        match cpu.resume() {
            CpuResult::Input => cpu.input(phase),
            _ => unreachable!(),
        }
        cpus.push(cpu);
    }

    let mut value = 0;
    let mut i = 0;
    loop {
        match cpus[i].resume() {
            CpuResult::Halt => break,
            CpuResult::Input => cpus[i].input(value),
            CpuResult::Output(out) => {
                value = out;
                i = (i + 1) % cpus.len();
            }
        }
    }

    value
}

fn find_max_sequence(phases: &[i64]) -> i64 {
    // Get the output for each permutation of phases, and choose the largest
    permutations_of(phases).map(try_sequence).max().unwrap()
}

fn main() {
    println!("Part 1: {}", find_max_sequence(&[0, 1, 2, 3, 4]));
    println!("Part 2: {}", find_max_sequence(&[5, 6, 7, 8, 9]));
}
