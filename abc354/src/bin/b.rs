use itertools::Itertools;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        n: usize,
        mut sc: [(Chars, usize); n],
    }
    sc.sort();
    let p = sc.iter().map(|(_, num)| num).sum::<usize>() % n;
    println!("{}", &sc[p].0.iter().join(""));
}
