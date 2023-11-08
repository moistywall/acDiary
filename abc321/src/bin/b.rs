use proconio::{
    input,
};

fn main() {
    input! {
        (n, x): (usize, i64),
        mut a: [i64; n - 1],
    }

    a.sort();
    let sum: i64 = a.iter().sum();
    let ps = sum - a[0] - a.last().unwrap();

    let mut ans = x - ps;
    let last = *a.last().unwrap();
    if ans > last {
        println!("-1");
        return;
    }
    if ans <= a[0] { ans = 0; }
    println!("{}", ans);
}
