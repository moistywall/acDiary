use itertools::Itertools;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        a: Chars,
    }
    let b = a.iter().map(|&ai| ai as u8 - b'0').collect_vec();
    let mut ans = 0usize;
    for bi in b {
        let ci = bi as usize;
        ans += ci * 100;
        ans += ci * 10;
        ans += ci;
    }
    println!("{}", ans);
}
