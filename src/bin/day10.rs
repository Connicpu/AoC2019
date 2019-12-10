use std::collections::HashSet;

use num::Integer;

static INPUT: &str = include_str!("input/day10.txt");

fn parse() -> impl Iterator<Item = &'static [u8]> {
    INPUT.lines().map(|line| line.as_bytes())
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn diff(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn gcd_form(self) -> Vec2 {
        let gcd = self.x.gcd(&self.y);
        Vec2 {
            x: (self.x / gcd).abs(),
            y: (self.y / gcd).abs(),
        }
    }

    fn between(self, a: Vec2, b: Vec2) -> bool {
        self.x >= a.x.min(b.x)
            && self.x <= a.x.max(b.x)
            && self.y >= a.y.min(b.y)
            && self.y <= a.y.max(b.y)
    }

    fn occludes(self, a: Vec2, b: Vec2) -> bool {
        if !self.between(a, b) {
            return false;
        }

        let asdiff = self.diff(a);
        let abdiff = b.diff(a);

        asdiff.gcd_form() == abdiff.gcd_form()
    }

    fn angle(self, b: Vec2) -> f64 {
        use std::f64::consts::{FRAC_PI_2, PI};
        let diff = b.diff(self);
        let theta = FRAC_PI_2 - (-diff.y as f64).atan2(diff.x as f64);
        theta.rem_euclid(PI * 2.0)
    }
}

fn asteroids() -> HashSet<Vec2> {
    let mut asteroids = HashSet::new();
    for (y, line) in parse().enumerate() {
        for (x, &slot) in line.iter().enumerate() {
            if slot == b'#' {
                let (x, y) = (x as i32, y as i32);
                asteroids.insert(Vec2 { x, y });
            }
        }
    }
    asteroids
}

fn can_see(a: Vec2, b: Vec2, asteroids: &HashSet<Vec2>) -> bool {
    for &c in asteroids {
        if c != a && c != b && c.occludes(a, b) {
            return false;
        }
    }
    true
}

fn all_detectable(a: Vec2, asteroids: &'_ HashSet<Vec2>) -> impl Iterator<Item = Vec2> + '_ {
    asteroids
        .iter()
        .cloned()
        .filter(move |&b| b != a && can_see(a, b, asteroids))
}

fn best_station(asteroids: &HashSet<Vec2>) -> (Vec2, usize) {
    asteroids
        .iter()
        .map(|&a| (a, all_detectable(a, &asteroids).count()))
        .max_by_key(|(_, v)| *v)
        .unwrap()
}

fn vaporize(monitor: Vec2, asteroids: &mut HashSet<Vec2>) -> Vec2 {
    let mut count = 0;
    let mut detectable = Vec::with_capacity(100);
    loop {
        detectable.extend(all_detectable(monitor, asteroids));
        detectable.sort_by(|&a, &b| monitor.angle(a).partial_cmp(&monitor.angle(b)).unwrap());

        if count + detectable.len() < 200 {
            count += detectable.len();
            for asteroid in detectable.drain(..) {
                asteroids.remove(&asteroid);
            }
        } else {
            let remaining = 200 - count;
            // we must return the 200th asteroid to be vaporized
            return detectable[remaining - 1];
        }
    }
}

fn main() {
    let mut asteroids = asteroids();

    let (monitor, detectable) = best_station(&asteroids);
    println!("Part 1: {}", detectable);

    let p2 = vaporize(monitor, &mut asteroids);
    println!("Part 2: {}", p2.x * 100 + p2.y);
}
