use aoc2019::intcode::{Cpu, IO};

use std::collections::HashMap;
use std::ops::{Add, Sub};

static INPUT: &str = include_str!("input/day11.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        (self.x + other.x, self.y + other.y).into()
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Vec2 {
        Vec2 { x, y }
    }
}

static DIRS: [Vec2; 4] = [
    Vec2 { x: 0, y: -1 },
    Vec2 { x: 1, y: 0 },
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
];

fn color(pos: Vec2, hull: &HashMap<Vec2, i64>) -> i64 {
    *hull.get(&pos).unwrap_or(&0)
}

fn paint(hull: &mut HashMap<Vec2, i64>) {
    #[derive(Copy, Clone)]
    enum State {
        Paint,
        Rotate,
    }

    struct Painter<'a> {
        hull: &'a mut HashMap<Vec2, i64>,
        pos: Vec2,
        dir: usize,
        state: State,
    }

    impl IO for Painter<'_> {
        fn input(&mut self) -> i64 {
            color(self.pos, self.hull)
        }

        fn output(&mut self, value: i64) {
            self.state = match self.state {
                State::Paint => {
                    self.hull.insert(self.pos, value);
                    State::Rotate
                }
                State::Rotate => {
                    self.dir = (self.dir + value as usize * 2 + 3).rem_euclid(4);
                    self.pos = self.pos + DIRS[self.dir];
                    State::Paint
                }
            }
        }
    }

    Cpu::parse(INPUT).run(Painter {
        hull,
        pos: Vec2::default(),
        dir: 0,
        state: State::Paint,
    });
}

fn bounds(hull: &HashMap<Vec2, i64>) -> (i32, i32, i32, i32) {
    let min_x = hull.keys().map(|k| k.x).min().unwrap();
    let max_x = hull.keys().map(|k| k.x).max().unwrap();
    let min_y = hull.keys().map(|k| k.y).min().unwrap();
    let max_y = hull.keys().map(|k| k.y).max().unwrap();
    (min_x, max_x, min_y, max_y)
}

fn main() {
    // Part 1
    let mut hull = HashMap::new();
    paint(&mut hull);
    println!("Part 1: {}", hull.len());

    // Calculate Part 2
    let mut hull = HashMap::new();
    hull.insert(Vec2 { x: 0, y: 0 }, 1);
    paint(&mut hull);

    // Paint Part 2
    let (min_x, max_x, min_y, max_y) = bounds(&hull);
    println!("Part 2:");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match color((x, y).into(), &hull) {
                1 => '#',
                _ => ' ',
            };
            print!("{}{}", c, c);
        }
        println!();
    }
}
