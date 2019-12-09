use aoc2019::intcode::Cpu;

static INPUT: &str = include_str!("input/day02.txt");

fn opcodes() -> Vec<i64> {
    INPUT.split(',').filter_map(|i| i.parse().ok()).collect()
}

fn process(noun: i64, verb: i64) -> i64 {
    let mut cpu = Cpu::new(opcodes());
    cpu.memory[1] = noun;
    cpu.memory[2] = verb;
    cpu.run();
    cpu.memory[0]
}

fn main() {
    // Part 1
    println!("Part 1: {}", process(12, 2));

    // Part 2
    'part2: for noun in 0..=99 {
        for verb in 0..=99 {
            if process(noun, verb) == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break 'part2;
            }
        }
    }
}
