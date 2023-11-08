use proconio::{
    input,
    marker::Bytes,
};

fn main() {
    input! {
        s: Bytes,
    }
    let ans = s.iter().position(|&si| si < 91).unwrap();
    println!("{}", ans + 1);
}
