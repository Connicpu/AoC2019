use aoc2019::intcode::{ChannelIO, Cpu, IterIO};

static INPUT: &str = include_str!("input/day07.txt");

fn make_cpu(phase: i32, input: i32) -> Cpu<IterIO<impl Iterator<Item = i32>>> {
    Cpu::parse_with_io(
        INPUT,
        IterIO::new(Some(phase).into_iter().chain(Some(input))),
    )
}

fn try_sequence(seq: &[&i32]) -> i32 {
    let mut input = 0;
    for &&phase in seq {
        let mut cpu = make_cpu(phase, input);
        cpu.run();
        input = cpu.io.output
    }
    input
}

fn bigger_sequence(seq: &[&i32]) -> i32 {
    use std::sync::mpsc::channel;
    use std::thread::spawn;

    let (a_tx, a_rx) = channel();
    let (b_tx, b_rx) = channel();
    let (c_tx, c_rx) = channel();
    let (d_tx, d_rx) = channel();
    let (e_tx, e_rx) = channel();

    a_tx.send(*seq[0]).unwrap();
    b_tx.send(*seq[1]).unwrap();
    c_tx.send(*seq[2]).unwrap();
    d_tx.send(*seq[3]).unwrap();
    e_tx.send(*seq[4]).unwrap();

    a_tx.send(0).unwrap();

    spawn(move || {
        Cpu::parse_with_io(INPUT, ChannelIO::new(a_rx, b_tx)).run();
    });
    spawn(move || {
        Cpu::parse_with_io(INPUT, ChannelIO::new(b_rx, c_tx)).run();
    });
    spawn(move || {
        Cpu::parse_with_io(INPUT, ChannelIO::new(c_rx, d_tx)).run();
    });
    spawn(move || {
        Cpu::parse_with_io(INPUT, ChannelIO::new(d_rx, e_tx)).run();
    });

    let mut cpu_e = Cpu::parse_with_io(INPUT, ChannelIO::new(e_rx, a_tx));
    cpu_e.run();
    cpu_e.io.last_output
}

fn main() {
    let permutations = permutator::XPermutationIterator::new(&[0, 1, 2, 3, 4], |_| true);
    println!(
        "Part 1: {}",
        permutations
            .map(|p| try_sequence(&p))
            .max()
            .expect("There's a max...")
    );

    let permutations = permutator::XPermutationIterator::new(&[5, 6, 7, 8, 9], |_| true);
    println!(
        "Part 2: {}",
        permutations
            .map(|p| bigger_sequence(&p))
            .max()
            .expect("There's a max...")
    );
}
