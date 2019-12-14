use aoc2019::intcode::Cpu;

static INPUT: &str = include_str!("input/day09.txt");

fn compute_boost(input: i64) -> i64 {
    Cpu::parse(INPUT).compute(input)
}

fn main() {
    println!("Part 1: {}", compute_boost(1));
    println!("Part 2: {}", compute_boost(2));
}
