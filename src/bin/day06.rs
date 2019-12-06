use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

static INPUT: &str = include_str!("input/day06.txt");

#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
struct Id([u8; 3]);

type OrbitMap = HashMap<Id, Vec<Id>>;
type OrbitTree = HashMap<Id, Node>;

fn orbits() -> Vec<(Id, Id)> {
    let mut orbits = Vec::with_capacity(1100);
    for line in INPUT.lines() {
        let line = line.as_bytes();
        if line.len() < 7 {
            continue;
        }

        let a = Id([line[0], line[1], line[2]]);
        let b = Id([line[4], line[5], line[6]]);
        orbits.push((a, b));
    }
    orbits
}

fn orbit_map(orbits: &[(Id, Id)]) -> HashMap<Id, Vec<Id>> {
    let mut maps = OrbitMap::with_capacity(orbits.len());

    for &(a, b) in orbits {
        maps.entry(a).or_default().push(b);
    }

    maps
}

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

fn connected_tree(id: &Id, tree: &OrbitTree) -> Vec<(Id, i32)> {
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

fn count_suborbits(map: &OrbitMap, id: Id, depth: i32) -> i32 {
    if !map.contains_key(&id) {
        return 0;
    }

    let mut count = 0;
    for &obj in &map[&id] {
        count += depth;
        count += count_suborbits(map, obj, depth + 1);
    }
    count
}

fn main() {
    let orbits = orbits();
    let orbit_map = orbit_map(&orbits);
    let tree = orbit_tree(&orbits);
    let com = Id([b'C', b'O', b'M']);
    println!("Part 1: {}", count_suborbits(&orbit_map, com, 1));

    let you = Id([b'Y', b'O', b'U']);
    let san = Id([b'S', b'A', b'N']);
    let you_parent = tree[&you].parent;
    let san_parent = tree[&san].parent;
    let result = dijkstra(&you_parent, |n| connected_tree(n, &tree), |n| *n == san_parent);
    println!("Part 2: {:?}", result.map(|(_, c)| c));
}
