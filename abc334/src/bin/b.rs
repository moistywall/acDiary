use proconio::{
    input,
};

fn main() {
    input! {
        (a, m, l, r): (isize, isize, isize, isize),
    }
    let rr = (r - a) / m;
    let mut ll = (a - l) / m;
    if a >= l && a <= r {
        ll += 1;
    } else if a < l && (a - l) % m == 0 {
        ll += 1;
    } else if a > r && (a - r) % m == 0 {
        ll += 1;
    }
    println!("{}", rr + ll);
}
