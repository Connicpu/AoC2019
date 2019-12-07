use aoc2019::intcode::{ChannelIO, Cpu};

use std::sync::mpsc::channel;
use std::thread::spawn;

use permute::permutations_of;

static INPUT: &str = include_str!("input/day07.txt");

fn try_sequence<'a>(seq: impl Iterator<Item = &'a i32>) -> i32 {
    let mut senders = Vec::with_capacity(5);
    let mut receivers = Vec::with_capacity(5);

    // Create a channel pair for each CPU
    for &phase in seq {
        let (tx, rx) = channel();
        tx.send(phase).unwrap();
        senders.push(tx);
        receivers.push(rx);
    }

    // Send the initial input to the first CPU
    senders[0].send(0).unwrap();

    // Rotate the senders so that CPU A's output will send to CPU B's input
    senders.rotate_left(1);

    // Construct the CPUs in a feedback loop. The feedback loop
    // will be ignored for part 1 as each CPU will merely halt
    // when it computes its result.
    let mut cpus = Vec::with_capacity(5);
    for (rx, tx) in receivers.into_iter().zip(senders) {
        cpus.push(Cpu::parse_with_io(INPUT, ChannelIO::new(rx, tx)));
    }

    // Run all but the last CPU on its own thread
    for mut cpu in cpus.drain(..cpus.len() - 1) {
        spawn(move || cpu.run());
    }

    // Run the final CPU and get its output when it finally halts
    cpus[0].run();
    cpus[0].io.last_output
}

fn find_max_sequence(phases: &[i32]) -> i32 {
    // Get the output for each permutation of phases, and choose the largest
    permutations_of(phases).map(try_sequence).max().unwrap()
}

fn main() {
    println!("Part 1: {}", find_max_sequence(&[0, 1, 2, 3, 4]));
    println!("Part 2: {}", find_max_sequence(&[5, 6, 7, 8, 9]));
}
