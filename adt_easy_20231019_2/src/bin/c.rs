use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut s: Bytes,
    }
    s.sort();
    let chk1 = s.iter().any(|&si| si < 91);
    let chk2 = s.iter().any(|&si| si > 91);
    let len1 = s.len();
    s.dedup();
    let chk3 = len1 == s.len();
    println!("{}", if chk1 && chk2 && chk3 { "Yes" } else { "No" });
}
