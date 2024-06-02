use proconio::{
    input,
};

fn main() {
    input! {
        (a, b, c): (usize, u32, usize),
    }
    let l = a;
    let r = c.pow(b);
    // println!("{} {}", l, r);
    println!("{}", if l < r { "Yes" } else { "No" });
}
