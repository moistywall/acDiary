use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [(usize, usize, usize, usize); n - 2],
    }
}
