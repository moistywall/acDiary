use proconio::{
    input,
};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
    }

    let l = lcm(n, m);
    let mut ok = usize::MAX;
    let mut ng = 0usize;
    while ok != ng {
        let mid = (ok + ng) / 2;
        let num = (mid / n) + (mid / m) - (mid / l) * 2;
        if num >= k {
            ok = mid;
        } else {
            ng = mid + 1;
        }
    }
    println!("{}", ok);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { return a; }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    a * b / g
}
