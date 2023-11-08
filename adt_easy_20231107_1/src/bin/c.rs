use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut a = vec![vec![]; n];
    a[0].push(1usize);
    for i in 1..n {
        for j in 0..=i {
            if j == 0 || j == i {
                a[i].push(1);
            } else {
                let next = a[i - 1][j - 1] + a[i - 1][j];
                a[i].push(next);
            }
        }
    }
    for ai in a {
        println!("{}", ai.iter().join(" "));
    }
}
