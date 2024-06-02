use std::collections::BTreeSet;
use proconio::{
    input,
    marker::Chars,
};


fn main() {
    input! {
        mut s: Chars
    }

    let n = s.len();
    let mut ans = BTreeSet::new();
    for i in 1..=n {
        for si in s.windows(i) {
            ans.insert(si);
        }
    }
    println!("{}", ans.len());
}
