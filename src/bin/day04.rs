const MIN: i32 = 134792;
const MAX: i32 = 675810;

fn valid(pwd: i32) -> bool {
    let mut buf = [0; 6];
    itoa::write(&mut buf[..], pwd).expect("um");

    // two digits must be consecutive
    if !buf.windows(2).any(|w| w[0] == w[1]) {
        return false;
    }

    // no decreases
    if !buf.windows(2).all(|w| w[0] <= w[1]) {
        return false;
    }
    true
}

fn extra_valid(pwd: i32) -> bool {
    if !valid(pwd) {
        return false;
    }

    let mut buf = [0; 6];
    itoa::write(&mut buf[..], pwd).expect("um");

    let mut range_c = 0;
    let mut range_len = 0;
    let mut has_twofer = false;
    for i in 0..6 {
        if buf[i] != range_c {
            if range_len == 2 {
                has_twofer = true;
                break;
            }
            range_c = buf[i];
            range_len = 0;
        }
        range_len += 1;
    }
    if range_len == 2 {
        has_twofer = true;
    }

    has_twofer
}

fn input() -> impl Iterator<Item = i32> {
    MIN..=MAX
}

fn count_pwds(f: impl Fn(i32) -> bool) -> usize {
    input().filter(|i| f(*i)).count()
}

fn main() {
    println!("Part 1: {}", count_pwds(valid));
    println!("Part 2: {}", count_pwds(extra_valid));
}
