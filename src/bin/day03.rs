static INPUT: &str = include_str!("input/day03.txt");

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Copy, Clone)]
struct Segment {
    a: Point,
    b: Point,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Alignment {
    Horiz,
    Vert,
}

impl Segment {
    fn alignment(self) -> Alignment {
        if self.a.x == self.b.x {
            Alignment::Vert
        } else {
            Alignment::Horiz
        }
    }

    fn len(self) -> i32 {
        self.a.dist(self.b)
    }

    fn intersect(self, other: Segment) -> Option<Point> {
        let align = self.alignment();
        if align == other.alignment() {
            return None;
        }

        match align {
            Alignment::Horiz if self.crosses_horiz(other) => Some(Point {
                x: other.a.x,
                y: self.a.y,
            }),
            Alignment::Vert if other.crosses_horiz(self) => Some(Point {
                x: self.a.x,
                y: other.a.y,
            }),
            _ => None,
        }
    }

    fn left(self) -> i32 {
        self.a.x.min(self.b.x)
    }
    fn right(self) -> i32 {
        self.a.x.max(self.b.x)
    }
    fn top(self) -> i32 {
        self.a.y.max(self.b.y)
    }
    fn bottom(self) -> i32 {
        self.a.y.min(self.b.y)
    }

    fn crosses_horiz(self, other: Segment) -> bool {
        other.a.x > self.left()
            && other.a.x < self.right()
            && other.top() > self.a.y
            && other.bottom() < self.a.y
    }
}

struct Wire {
    segments: Vec<Segment>,
}

fn wire(data: &str) -> Wire {
    let mut pos = Point { x: 0, y: 0 };
    let mut segments = vec![];
    for dir in data.split(',') {
        let dist = dir[1..].parse::<i32>().unwrap();
        let next = match dir.as_bytes()[0] {
            b'U' => Point {
                x: pos.x,
                y: pos.y + dist,
            },
            b'D' => Point {
                x: pos.x,
                y: pos.y - dist,
            },
            b'L' => Point {
                x: pos.x - dist,
                y: pos.y,
            },
            b'R' => Point {
                x: pos.x + dist,
                y: pos.y,
            },
            _ => panic!(),
        };

        segments.push(Segment { a: pos, b: next });
        pos = next;
    }
    Wire { segments }
}

fn wires() -> Vec<Wire> {
    INPUT.lines().map(wire).collect()
}

fn main() {
    use std::i32::MAX;

    let wires = wires();

    let origin = Point { x: 0, y: 0 };
    let mut dist = MAX;
    for seg_a in wires[0].segments.iter() {
        for seg_b in wires[1].segments.iter() {
            if let Some(intersection) = seg_a.intersect(*seg_b) {
                dist = intersection.dist(origin).min(dist);
            }
        }
    }

    println!("Part 1: {}", dist);

    let mut lowest_steps = MAX;
    let mut a_steps = 0;
    for seg_a in wires[0].segments.iter() {
        let mut b_steps = 0;
        for seg_b in wires[1].segments.iter() {
            if let Some(intersection) = seg_a.intersect(*seg_b) {
                let ias = intersection.dist(seg_a.a);
                let ibs = intersection.dist(seg_b.a);

                let total_steps = a_steps + b_steps + ias + ibs;
                lowest_steps = lowest_steps.min(total_steps);
            }
            b_steps += seg_b.len();
        }
        a_steps += seg_a.len();
    }

    println!("Part 2: {}", lowest_steps);
}
