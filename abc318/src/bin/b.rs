use itertools::Itertools;
use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [(usize, usize, usize, usize); n],
    }

    let mut v = vec![false; 10000];
    for (a, b, c, d) in a {
        for j in c..d { for i in a..b {
            v[i + 100 * j] = true;
        }}
    }
    println!("{}", v.iter().filter(|v| **v).collect_vec().len());
}
