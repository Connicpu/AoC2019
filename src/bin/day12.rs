use aoc2019::parse::parse;

use std::ops::{Add, Sub};

use num::Integer;

static INPUT: &[u8] = include_bytes!("input/day12.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn signum(self) -> Vec3 {
        (self.x.signum(), self.y.signum(), self.z.signum()).into()
    }

    fn energy(self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        (self.x + rhs.x, self.y + rhs.y, self.z + rhs.z).into()
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        (self.x - rhs.x, self.y - rhs.y, self.z - rhs.z).into()
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Vec3 {
        Vec3 { x, y, z }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn energy(self) -> i32 {
        self.pos.energy() * self.vel.energy()
    }
}

fn parse_moons() -> Vec<Moon> {
    let mut positions = parse::<i32>(INPUT);
    let mut moons = Vec::new();
    while let Some(x) = positions.next() {
        let y = positions.next().unwrap();
        let z = positions.next().unwrap();

        let pos = (x, y, z).into();
        let vel = Default::default();
        moons.push(Moon { pos, vel })
    }
    moons
}

fn apply_gravity(moons: &mut [Moon]) {
    for a in 1..moons.len() {
        for b in 0..a {
            let dir = (moons[a].pos - moons[b].pos).signum();
            moons[a].vel = moons[a].vel - dir;
            moons[b].vel = moons[b].vel + dir;
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    for moon in moons {
        moon.pos = moon.pos + moon.vel;
    }
}

fn calc1000() -> i32 {
    let mut moons = parse_moons();

    for _ in 0..1000 {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }

    moons.iter().map(|moon| moon.energy()).sum()
}

fn axis_eq(a: &[Moon], b: &[Moon], axis: impl Fn(Vec3) -> i32) -> bool {
    for (a, b) in a.iter().zip(b.iter()) {
        if axis(a.pos) != axis(b.pos) || axis(a.vel) != axis(b.vel) {
            return false;
        }
    }
    true
}

fn find_axis_cycle(initial: &[Moon], axis: impl Fn(Vec3) -> i32) -> u64 {
    let mut state = Vec::from(initial);
    let mut steps = 0;

    loop {
        apply_gravity(&mut state);
        apply_velocity(&mut state);
        steps += 1;

        if axis_eq(&state, &initial, &axis) {
            break;
        }
    }

    steps
}

fn find_repeated_state() -> u64 {
    let initial = parse_moons();
    
    let x_steps = find_axis_cycle(&initial, |v| v.x);
    let y_steps = find_axis_cycle(&initial, |v| v.y);
    let z_steps = find_axis_cycle(&initial, |v| v.z);

    x_steps.lcm(&y_steps).lcm(&z_steps)
}

fn main() {
    println!("Part 1: {}", calc1000());
    println!("Part 2: {}", find_repeated_state());
}
