use std::cmp::Ordering;
use std::collections::HashSet;
use std::f64::consts::{FRAC_PI_2, PI};
use std::ops::Sub;

use num::Integer;

static INPUT: &str = include_str!("input/day10.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    /// Converts this vector into its simplified form, dividing
    /// it by gcd(x, y).
    fn simplified(self) -> Vec2 {
        let gcd = self.x.gcd(&self.y);
        (self.x / gcd, self.y / gcd).into()
    }

    /// Determines if the asteroid at this position is possibly
    /// between the asteroids at `a` and `b`, in an axis-aligned way.
    fn between(self, a: Vec2, b: Vec2) -> bool {
        self.x >= a.x.min(b.x)
            && self.x <= a.x.max(b.x)
            && self.y >= a.y.min(b.y)
            && self.y <= a.y.max(b.y)
    }

    /// Determines if the asteroid at this position would block the line of
    /// sight between `a` and `b`
    fn occludes(self, a: Vec2, b: Vec2) -> bool {
        self.between(a, b) && (self - a).simplified() == (b - a).simplified()
    }

    /// Determines the angle between these vecs from the up vector, moving clockwise
    fn angle(self, b: Vec2) -> f64 {
        let diff = b - self;
        let theta = FRAC_PI_2 - (-diff.y as f64).atan2(diff.x as f64);
        theta.rem_euclid(PI * 2.0)
    }

    /// Compare the angles between (self,a) and (self,b)
    fn cmp_angles(self, a: Vec2, b: Vec2) -> Ordering {
        self.angle(a).partial_cmp(&self.angle(b)).unwrap()
    }

    /// Determines if `self` has line of sight to `other`, given all
    /// the other asteroids in the field.
    fn can_see(self, other: Vec2, asteroids: &HashSet<Vec2>) -> bool {
        for &occluder in asteroids {
            if occluder == self || occluder == other {
                continue;
            }
            if occluder.occludes(self, other) {
                return false;
            }
        }
        true
    }

    /// Enumerates all asteroids which have line of sight from `self`
    fn all_detectable(self, asteroids: &'_ HashSet<Vec2>) -> impl Iterator<Item = Vec2> + '_ {
        asteroids
            .iter()
            .cloned()
            .filter(move |&b| b != self && self.can_see(b, asteroids))
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

/// Parse all of the asteroids into a set
fn asteroids() -> HashSet<Vec2> {
    let mut asteroids = HashSet::new();
    let data = INPUT.lines().map(|line| line.as_bytes());
    for (y, line) in data.enumerate() {
        for (x, &slot) in line.iter().enumerate() {
            if slot == b'#' {
                let (x, y) = (x as i32, y as i32);
                asteroids.insert((x, y).into());
            }
        }
    }
    asteroids
}

/// Given all of the asteroids in the field, find the one which
/// has the most detectable asteroids. Returns that asteroid and
/// how many it can detect.
fn best_station(asteroids: &HashSet<Vec2>) -> (Vec2, usize) {
    asteroids
        .iter()
        .map(|&a| (a, a.all_detectable(asteroids).count()))
        .max_by_key(|(_, v)| *v)
        .unwrap()
}

/// Vaporize all of the asteroids in a clockwise motion until the
/// 200th, returning the position of the 200th asteroid to be vaporized.
fn vaporize(monitor: Vec2, asteroids: &mut HashSet<Vec2>) -> Vec2 {
    let mut count = 0;
    let mut detectable = Vec::with_capacity(100);
    loop {
        detectable.extend(monitor.all_detectable(asteroids));
        detectable.sort_by(|&a, &b| monitor.cmp_angles(a, b));

        if count + detectable.len() < 200 {
            count += detectable.len();
            for asteroid in detectable.drain(..) {
                asteroids.remove(&asteroid);
            }
        } else {
            let remaining = 200 - count;
            // We must return the 200th asteroid to be vaporized
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
