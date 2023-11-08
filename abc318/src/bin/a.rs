use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        (n, m, p): (usize, usize, usize),
    }

    if n < m {
        println!("0");
        return ();
    }
    let ans = (n - m) / p + 1;
    println!("{}", ans);
}
