use proconio::{
    input,
};

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
    }
    for ai in a {
        if ai % k == 0 { print!("{} ", ai / k); }
    }
}
