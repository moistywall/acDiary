use std::collections::BTreeSet;
use itertools::Itertools;
use proconio::{
    input,
};

fn main() {
    input! {
        m: usize,
        mut ac: [(usize, usize); m],
    }
    let mut num = (1..m+1).collect::<BTreeSet<usize>>();
    ac.sort_by(|a, b| b.0.cmp(&a.0));
    println!("{:?}", num);
    println!("{:?}", ac);
    for i in 0..m {
        for j in 0..m {
            if ac[i].0 == ac[j].0 { continue; }
            if ac[i].1 < ac[j].1 {
                num.remove(&(j + 1));
            }
        }
    }
    println!("{}", num.iter().join(" "));
}
