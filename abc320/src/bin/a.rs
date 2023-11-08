use proconio::{
    input,
};

fn main() {
    input! {
        (a, b): (usize, usize),
    }
    let mut a1 = 1;
    let mut a2 = 1;
    for _ in 0..b {
        a1 *= a;
    }
    for _ in 0..a {
        a2 *= b;
    }
    println!("{}", a1 + a2);
}
