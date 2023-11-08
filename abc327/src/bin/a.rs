use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let ans = s.windows(2).any(|a| (a[0] == 'a' && a[1] == 'b') || (a[1] == 'a' && a[0] =='b'));
    println!("{}", if ans { "Yes" } else { "No" });
}
