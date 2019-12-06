use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

static INPUT: &str = include_str!("input/day06.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
struct Id(&'static str);

fn orbits() -> Vec<(Id, Id)> {
    let mut orbits = Vec::with_capacity(1100);
    for line in INPUT.lines() {
        if line.len() < 7 {
            continue;
        }

        let a = Id(&line[0..3]);
        let b = Id(&line[4..7]);
        orbits.push((a, b));
    }
    orbits
}

type OrbitTree = HashMap<Id, Node>;

#[derive(Default)]
struct Node {
    parent: Id,
    children: Vec<Id>,
}

fn orbit_tree(orbits: &[(Id, Id)]) -> OrbitTree {
    let mut tree = OrbitTree::with_capacity(orbits.len());

    for &(a, b) in orbits {
        let node_a = tree.entry(a).or_default();
        node_a.children.push(b);

        let node_b = tree.entry(b).or_default();
        node_b.parent = a;
    }

    tree
}

fn adjacent_bodies(id: &Id, tree: &OrbitTree) -> Vec<(Id, i32)> {
    if let Some(node) = tree.get(id) {
        node.children
            .iter()
            .chain(Some(&node.parent))
            .cloned()
            .map(|n| (n, 1))
            .collect()
    } else {
        vec![]
    }
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
    println!("Part 1: {}", count_suborbits(&tree, Id("COM"), 1));

    // Part 2
    let start = tree[&Id("YOU")].parent;
    let end = tree[&Id("SAN")].parent;

    let result = dijkstra(&start, |n| adjacent_bodies(n, &tree), |n| *n == end);

    println!("Part 2: {:?}", result.map(|(_, c)| c));
}
