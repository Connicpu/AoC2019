use aoc2019::intcode::{Cpu, CpuResult::*};

use std::collections::HashMap;
use std::ops::{Add, Sub};

use pathfinding::prelude::dijkstra;

static INPUT: &str = include_str!("input/day15.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn signum(self) -> Vec2 {
        (self.x.signum(), self.y.signum()).into()
    }
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
    Vec2 { x: 0, y: 1 },
    Vec2 { x: -1, y: 0 },
    Vec2 { x: 1, y: 0 },
];

#[derive(Default)]
struct State {
    pos: Vec2,
    next: Vec2,
    end: Vec2,
    map: HashMap<Vec2, i32>,
    path: Vec<Vec2>,
    awaiting_update: bool,
}

impl State {
    fn path_length(&mut self, start: Vec2, end: Vec2) -> i32 {
        let (_path, cost) = dijkstra(
            &start,
            |&pos| self.neighbors(pos).map(|p| (p, 1)),
            |&pos| pos == end,
        )
        .unwrap();

        cost
    }

    fn has_next(&mut self) -> bool {
        if self.awaiting_update {
            return true;
        }

        if self.path.is_empty() {
            let result = dijkstra(
                &self.pos,
                |&pos| self.neighbors(pos).map(|p| (p, 1)),
                |pos| !self.map.contains_key(pos),
            );
            let (path, _cost) = match result {
                Some(res) => res,
                None => return false,
            };
            self.path = path;
            self.path.remove(0);
            true
        } else {
            true
        }
    }

    fn next_input(&mut self) -> i64 {
        self.awaiting_update = true;

        let dir = (self.path[0] - self.pos).signum();
        assert_eq!((dir.x + dir.y).abs(), 1);
        let dir_i = DIRS.iter().position(|&d| d == dir).unwrap();
        self.path.remove(0);
        self.next = self.pos + DIRS[dir_i];

        dir_i as i64 + 1
    }

    fn update(&mut self, result: i64) {
        self.awaiting_update = false;

        self.map.insert(self.next, result as i32);
        if result == 0 {
            self.path.clear();
        } else {
            self.pos = self.next;
            if result == 2 {
                self.end = self.next;
            }
        }
    }

    fn neighbors<'a>(&'a self, pos: Vec2) -> impl Iterator<Item = Vec2> + 'a {
        DIRS.iter()
            .map(move |&d| pos + d)
            .filter(move |p| self.map.get(p) != Some(&0))
    }

    /*fn paint(&self) {
        let minx = self.map.keys().map(|k| k.x).min().unwrap();
        let maxx = self.map.keys().map(|k| k.x).max().unwrap();
        let miny = self.map.keys().map(|k| k.y).min().unwrap();
        let maxy = self.map.keys().map(|k| k.y).max().unwrap();

        for y in miny..=maxy {
            for x in minx..=maxx {
                let pos = Vec2 { x, y };
                let val = *self.map.get(&pos).unwrap_or(&3);

                let c = match val {
                    0 => '#',
                    1 => '.',
                    2 => 'X',
                    3 => ' ',
                    4 => 'S',
                    5 => 'O',
                    _ => unreachable!(),
                };

                print!("{}", c);
            }
            println!();
        }
    }*/
}

fn main() {
    let mut state = State::default();
    let mut cpu = Cpu::parse(INPUT);

    state.update(1);

    while state.has_next() {
        match cpu.resume() {
            Halt => panic!(),
            Input => cpu.input(state.next_input()),
            Output(result) => state.update(result),
        }
    }

    let p1 = state.path_length((0, 0).into(), state.end);
    println!("Part 1: {}", p1);

    state.map.insert(state.end, 5);

    let mut spread = Vec::with_capacity(64);
    let mut any = true;
    let mut steps = 0;
    while any {
        for &pos in state.map.keys() {
            if state.map.get(&pos) != Some(&1) {
                continue;
            }
            for &dir in &DIRS {
                if state.map.get(&(pos + dir)) == Some(&5) {
                    spread.push(pos);
                }
            }
        }
        any = !spread.is_empty();
        if any {
            steps += 1;
        }
        for pos in spread.drain(..) {
            state.map.insert(pos, 5);
        }
    }
    println!("Part 2: {}", steps);
}
