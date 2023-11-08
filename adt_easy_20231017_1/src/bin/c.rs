use itertools::Itertools;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort();
    print!("{}", s.iter().join(""));
}
