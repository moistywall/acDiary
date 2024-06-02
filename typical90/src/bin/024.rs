use proconio::{
    input,
};

fn main() {
    input! {
        (n, k): (usize, usize),
        a: [usize; n],
        b: [usize; n],
    }

    let mut sum = 0usize;
    for (&a, &b) in a.iter().zip(b.iter()) {
        if a > b {
            sum += a - b;
        } else {
            sum += b - a;
        }
    }
    let buf = if sum > k {
        1
    } else {
        k - sum
    };
    println!("{}", if buf % 2 == 0 { "Yes" } else { "No" });
}
