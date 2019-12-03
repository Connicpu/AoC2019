static INPUT1: &str = include_str!("input/day01.txt");

fn inputs() -> impl Iterator<Item = i32> {
    INPUT1.lines().filter_map(|line| line.parse().ok())
}

fn fuel_req(mass: i32) -> i32 {
    (mass / 3 - 2).max(0)
}

fn total_fuel(mass: i32) -> i32 {
    let mut req = fuel_req(mass);
    let mut fuel = req;
    loop {
        fuel = fuel_req(fuel);
        if fuel > 0 {
            req += fuel;
        } else {
            break;
        }
    }
    req
}

fn main() {
    println!("Naive Sum {}", inputs().map(fuel_req).sum::<i32>());
    println!("Total Sum {}", inputs().map(total_fuel).sum::<i32>());
}
