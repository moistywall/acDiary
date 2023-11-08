use itertools::Itertools;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    for (i, si) in s.windows(3).enumerate() {
        if si[0] == 'A' && si[1] == 'B' && si[2] == 'C' {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
    return;
}
