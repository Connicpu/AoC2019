use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

static INPUT: &str = include_str!("input/day06.txt");

type Id = &'static str;

fn orbits() -> Vec<(Id, Id)> {
    let mut orbits = Vec::with_capacity(1100);
    for line in INPUT.lines() {
        if line.len() < 7 {
            continue;
        }

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
    let mut tree = OrbitTree::with_capacity(orbits.len());

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
