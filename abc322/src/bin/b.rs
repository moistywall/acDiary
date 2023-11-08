use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        (n, m): (usize, usize),
        s: Chars,
        t: Chars,
    }

    let mut ans = 0;
    if t[0..n] != s { ans += 2; }
    if t[m-n..m] != s { ans += 1; }
    println!("{}", ans);
}
