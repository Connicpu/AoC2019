#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2019::intcode::{parse, Cpu, IO};

use std::cmp::Ordering::*;
use std::time::Duration;

use once_cell::sync::Lazy;
use smallvec::SmallVec;

static INPUT: &str = include_str!("input/day13.txt");
static PROGRAM: Lazy<Vec<i64>> = Lazy::new(|| parse(INPUT));

const WIDTH: usize = 38;
const HEIGHT: usize = 21;

struct Screen {
    data: [u8; WIDTH * HEIGHT],
}

impl Screen {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x + y * WIDTH]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.data[x + y * WIDTH] = val;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen {
            data: [0; WIDTH * HEIGHT],
        }
    }
}

#[derive(Default)]
struct GameState {
    screen: Screen,
    score: i64,
    ball_x: i64,
    paddle_x: i64,

    out_buf: SmallVec<[i64; 2]>,

    draw: bool,
    draw_duration: Duration,
}

impl IO for GameState {
    fn output(&mut self, value: i64) {
        if self.out_buf.len() < 2 {
            self.out_buf.push(value);
            return;
        }

        let x = self.out_buf[0];
        let y = self.out_buf[1];
        self.out_buf.clear();

        if x == -1 && y == 0 {
            self.score = value;
        } else {
            if value == 3 {
                self.paddle_x = x;
            } else if value == 4 {
                self.ball_x = x;
            }

            self.screen.set(x as usize, y as usize, value as u8);
        }
    }

    fn input(&mut self) -> i64 {
        if self.draw {
            let start = std::time::Instant::now();
            print!("{}", term_cursor::Relative(0, -22));
            self.draw();
            let dur = std::time::Instant::now() - start;
            if dur < self.draw_duration {
                std::thread::sleep(self.draw_duration - dur);
            }
        }

        match self.ball_x.cmp(&self.paddle_x) {
            Less => -1,
            Equal => 0,
            Greater => 1,
        }
    }
}

impl GameState {
    fn draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let (c1, c2) = match self.screen.get(x, y) {
                    1 => ('█', '█'),
                    2 => ('▒', '▒'),
                    3 => ('▂', '▂'),
                    4 => ('▝', '▘'),
                    _ => (' ', ' '),
                };
                print!("{}{}", c1, c2);
            }
            println!();
        }
        let scorestr = format!(" Score: {:>5} ", self.score);
        println!("{:█^76}", scorestr);
    }
}

fn plain_run() -> usize {
    let mut cpu = Cpu::with_io(PROGRAM.clone(), GameState::default());
    cpu.run();

    cpu.io.screen.data.iter().filter(|&&v| v == 2).count()
}

fn dynamic_run(draw: bool, draw_duration: Duration) -> i64 {
    let state = GameState {
        draw,
        draw_duration,
        ..GameState::default()
    };

    if draw {
        state.draw();
    }

    let mut cpu = Cpu::with_io(PROGRAM.clone(), state);
    cpu.memory[0] = 2;
    cpu.run();

    cpu.io.score
}

fn main() {
    let draw = std::env::args()
        .nth(1)
        .map(|s| s == "draw")
        .unwrap_or(false);

    let draw_duration = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .map(Duration::from_millis)
        .unwrap_or(Duration::from_millis(0));

    println!("Part 1: {}", plain_run());
    println!("Part 2: {}", dynamic_run(draw, draw_duration));
}

#[cfg(test)]
#[bench]
fn part1(bench: &mut test::Bencher) {
    bench.iter(|| {
        test::black_box(plain_run());
    });
}

#[cfg(test)]
#[bench]
fn part2(bench: &mut test::Bencher) {
    bench.iter(|| {
        test::black_box(dynamic_run(false, Default::default()));
    });
}
