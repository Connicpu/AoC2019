use std::collections::HashMap;

static INPUT: &str = include_str!("input/day14.txt");

type Mat = &'static str;

#[derive(Debug, Copy, Clone)]
struct Quantity {
    amt: usize,
    mat: Mat,
}

type Requirement = [Quantity];
type Production = (Box<Requirement>, Quantity);
type Requirements<'a> = HashMap<Mat, (usize, &'a Requirement)>;
type Materials = HashMap<Mat, usize>;

fn parse_quant(quant: &'static str) -> Quantity {
    let mut parts = quant.trim().split(' ');
    let amt = parts.next().unwrap().parse().unwrap();
    let mat = parts.next().unwrap();

    Quantity { amt, mat }
}

fn parse_req(reqs: &'static str) -> Box<Requirement> {
    reqs.split(',').map(parse_quant).collect()
}

fn parse_prod(prod: &'static str) -> Production {
    let mut parts = prod.split("=>");
    let reqs = parts.next().unwrap().trim();
    let outp = parts.next().unwrap().trim();
    (parse_req(reqs), parse_quant(outp))
}

fn parse_productions() -> Vec<Production> {
    INPUT.lines().map(parse_prod).collect()
}

fn map_requirements(productions: &[Production]) -> Requirements {
    let mut reqs = Requirements::with_capacity(productions.len());
    for (req, out) in productions {
        reqs.insert(out.mat, (out.amt, req));
    }
    reqs
}

fn calculate_requirements(
    reqs: &Requirements,
    mut target: Quantity,
    excess: &mut Materials,
) -> usize {
    if target.mat == "ORE" {
        return target.amt;
    }

    if let Some(excess_amt) = excess.get_mut(target.mat) {
        if target.amt <= *excess_amt {
            *excess_amt -= target.amt;
            return 0;
        } else {
            target.amt -= *excess_amt;
        }
    }

    excess.insert(target.mat, 0);

    let mut ore = 0;
    let (output_amt, inputs) = reqs[target.mat];
    let operations = (target.amt - 1) / output_amt + 1;

    for input in inputs {
        let mut quant = *input;
        quant.amt *= operations;
        ore += calculate_requirements(reqs, quant, excess);
    }

    excess.insert(target.mat, output_amt * operations - target.amt);
    ore
}

fn part1(reqs: &Requirements) -> usize {
    let target = Quantity {
        amt: 1,
        mat: "FUEL",
    };

    let mut excess = Materials::new();
    calculate_requirements(reqs, target, &mut excess)
}

fn part2(reqs: &Requirements) -> usize {
    let mut ore = 1_000_000_000_000;
    let mut target_amt = ore;
    let mut fuel = 0;
    let mut excess = Materials::new();

    while ore > 0 && target_amt > 0 {
        let mut new_excess = excess.clone();
        let target = Quantity {
            mat: "FUEL",
            amt: target_amt,
        };
        let ore_used = calculate_requirements(reqs, target, &mut new_excess);
        // Check if too much; try half if it is, binary search it yo
        if ore_used > ore {
            target_amt /= 2;
        } else {
            fuel += target_amt;
            ore -= ore_used;
            excess = new_excess;
        }
    }

    fuel
}

fn main() {
    let prods = parse_productions();
    let reqs = map_requirements(&prods);

    println!("Part 1: {:?}", part1(&reqs));
    println!("Part 2: {:?}", part2(&reqs));
}
