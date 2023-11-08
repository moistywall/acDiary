use proconio::{
    input,
    marker::Bytes,
};

fn main() {
    input! {
        mut s: Bytes,
    }
    s.sort();
    for i in 0..=9 {
        if !s.contains(&(i + b'0')) { println!("{}", i)}
    }
}
