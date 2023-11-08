use itertools::Itertools;
use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        c: [usize; 9],
    }

    let deno = (0..9)
        .permutations(9)
        .filter(|v| {
            let mut row = vec![vec![]; 3];
            let mut col = vec![vec![]; 3];
            let mut d1 = vec![];
            let mut d2 = vec![];
            for &ij in v {
                let (i, j) = (ij / 3, ij % 3);
                row[i].push(c[ij]);
                col[j].push(c[ij]);
                if i == j { d1.push(c[ij]); }
                if i == (2 - j) { d2.push(c[ij]); }
            }
            !row.into_iter()
                .chain(col)
                .chain(vec![d1, d2])
                .any(|x| x[0] == x[1] && x[1] != x[2])
        })
        .count();
    let nume: usize = (1..=9).product();
    println!("{}", deno as f64 / nume as f64);
}

 /* 解１ https://atcoder.jp/contests/abc319/submissions/45361242 */