#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

static INPUT: &str = include_str!("input/day06.txt");

type Id = &'static str;

// Parse the orbits from the input file, it's just a (orbited body, orbiter)
// pair for every orbit relationship.
fn orbits() -> Vec<(Id, Id)> {
    // Save an allocation~ that's about how long the file is
    let mut orbits = Vec::with_capacity(1062);

    // lines() will spare us the final empty line :)
    for line in INPUT.lines() {
        // The bodies are all 3 character IDs, with a ) separating them
        let a = &line[0..3];
        let b = &line[4..7];
        orbits.push((a, b));
    }
    orbits
}

type OrbitTree = HashMap<Id, Node>;

#[derive(Default)]
struct Node {
    parent: Option<Id>,
    children: Vec<Id>,
}

fn orbit_tree(orbits: &[(Id, Id)]) -> OrbitTree {
    // This is apparently the exact capacity that will always be needed for
    // the exact size of our tree.
    let mut tree = OrbitTree::with_capacity(1792);

    for &(a, b) in orbits {
        let node_a = tree.entry(a).or_default();
        node_a.children.push(b);

        let node_b = tree.entry(b).or_default();
        node_b.parent = Some(a);
    }

    tree
}

fn adjacent_bodies(id: Id, tree: &'_ OrbitTree) -> impl Iterator<Item = (Id, i32)> + '_ {
    let node = &tree[&id];
    node.children
        .iter()
        .cloned()
        .chain(node.parent)
        .map(|n| (n, 1))
}

fn count_suborbits(map: &OrbitTree, id: Id, depth: i32) -> i32 {
    if !map.contains_key(&id) {
        return 0;
    }

    let mut count = 0;
    for &obj in &map[&id].children {
        count += depth;
        count += count_suborbits(map, obj, depth + 1);
    }
    count
}

fn main() {
    let orbits = orbits();
    let tree = orbit_tree(&orbits);

    // Part 1
    println!("Part 1: {}", count_suborbits(&tree, "COM", 1));

    // Part 2
    let start = tree["YOU"].parent.expect("YOU must have a parent");
    let end = tree["SAN"].parent.expect("SAN must have a parent");

    let (_path, cost) = dijkstra(&start, |&n| adjacent_bodies(n, &tree), |n| *n == end)
        .expect("There is no route from you to santa :(");

    println!("Part 2: {}", cost);
}

#[cfg(test)]
#[bench]
fn parsing(bench: &mut test::Bencher) {
    bench.iter(|| {
        let orbits = orbits();
        test::black_box(&orbits);
    });
}

#[cfg(test)]
#[bench]
fn tree_building(bench: &mut test::Bencher) {
    let orbits = orbits();
    bench.iter(|| {
        let tree = orbit_tree(&orbits);
        test::black_box(&tree);
    });
}

#[cfg(test)]
#[bench]
fn suborbit_counting(bench: &mut test::Bencher) {
    let orbits = orbits();
    let tree = orbit_tree(&orbits);
    bench.iter(|| {
        test::black_box(count_suborbits(&tree, "COM", 1));
    });
}

#[cfg(test)]
#[bench]
fn pathfinding(bench: &mut test::Bencher) {
    let orbits = orbits();
    let tree = orbit_tree(&orbits);
    bench.iter(|| {
        let start = tree["YOU"].parent.expect("YOU must have a parent");
        let end = tree["SAN"].parent.expect("SAN must have a parent");

        let (_path, cost) = dijkstra(&start, |&n| adjacent_bodies(n, &tree), |n| *n == end)
            .expect("There is no route from you to santa :(");
        test::black_box(cost);
    });
}
