use std::collections::BTreeSet;
use proconio::{
    input,
};

fn main() {
    input! {
        (n, m): (usize, usize),
    }
    let mut v = vec![BTreeSet::new(); n];
    for _ in 0..m {
        input! {
            k: usize,
            x: [usize; k],
        }
        for i in 0..k {
            for j in 0..k {
                v[x[i] - 1].insert(x[j] - 1);
            }
        }
    }

    let ans = v.iter().all(|vi| vi.len() == n);
    println!("{}", if ans { "Yes" } else { "No" });
}
