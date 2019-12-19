#![feature(test)]
extern crate test;

use std::mem::swap;

static INPUT: &str = include_str!("input/day16.txt");

static BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn pattern_at(p: usize, i: usize) -> i32 {
    let r = p + 1;
    let x = r + i;
    BASE_PATTERN[(x / r).rem_euclid(4)]
}

fn value_for(position: usize, input: &[i32]) -> i32 {
    input[position..]
        .iter()
        .enumerate()
        .map(|(i, &x)| x * pattern_at(position, i))
        .sum::<i32>()
        .abs()
        .rem_euclid(10)
}

fn run_phase(input: &[i32], output: &mut [i32]) {
    for (i, out) in output.iter_mut().enumerate() {
        *out = value_for(i, input);
    }
}

fn fft(input: &mut Vec<i32>, phases: usize) {
    let mut output = vec![0; input.len()];

    for _ in 0..phases {
        run_phase(&input, &mut output);
        swap(input, &mut output);
    }
}

fn cheaty_phase(input: &[i32], output: &mut [i32], offset: usize) {
    let len = input.len();
    output[len - 1] = input[len - 1];
    for i in (offset..len - 1).rev() {
        output[i] = (output[i + 1] + input[i]).abs().rem_euclid(10);
    }
}

// only works if offset is large ;)
fn cheaty_fft(input: &mut Vec<i32>, phases: usize, offset: usize) {
    let mut output = vec![0; input.len()];

    for _ in 0..phases {
        cheaty_phase(&input, &mut output, offset);
        swap(input, &mut output);
    }
}

fn parse() -> Vec<i32> {
    INPUT
        .trim()
        .as_bytes()
        .iter()
        .cloned()
        .map(|b| (b - b'0') as i32)
        .collect::<Vec<_>>()
}

fn main() {
    let mut input = parse();
    fft(&mut input, 100);
    print!("Part 1: ");
    for i in &input[0..8] {
        print!("{}", i);
    }
    println!();

    let mut input = Vec::new();
    let data = parse();
    let mut offset = 0;
    for &x in &data[0..7] {
        offset *= 10;
        offset += x as usize;
    }
    for _ in 0..10_000 {
        input.extend_from_slice(&data);
    }
    cheaty_fft(&mut input, 100, offset);
    print!("Part 2: ");
    for i in &input[offset..offset + 8] {
        print!("{}", i);
    }
    println!();
}

#[bench]
fn day16_p1_phase(bench: &mut test::Bencher) {
    let mut input = parse();
    let mut output = vec![0; input.len()];
    bench.iter(|| {
        run_phase(&input, &mut output);
        swap(&mut input, &mut output);
    });
}

#[bench]
fn day16_p2_phase(bench: &mut test::Bencher) {
    let mut input = Vec::new();
    let data = parse();
    let mut offset = 0;
    for &x in &data[0..7] {
        offset *= 10;
        offset += x as usize;
    }
    for _ in 0..10_000 {
        input.extend_from_slice(&data);
    }
    let mut output = vec![0; input.len()];
    bench.iter(|| {
        cheaty_phase(&input, &mut output, offset);
        swap(&mut input, &mut output);
    });
}
