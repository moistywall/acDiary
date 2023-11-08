use itertools::Itertools;
use proconio::{
    input,
};

const M: usize = usize::MAX;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut a: [usize; n],
        mut b: [usize; m],
    }
    a.push(M);
    b.push(M);

    let mut acnt = Vec::new();
    let mut bcnt = Vec::new();
    let mut ai = 0usize;
    let mut bi = 0usize;
    for i in 0..(n + m) {
        if a[ai] > b[bi] {
            bcnt.push(i + 1);
            bi += 1;
            if bi > m { bi = m; }
        } else {
            acnt.push(i + 1);
            ai += 1;
            if ai > n { ai = n; }
        }
    }
    println!("{}", acnt.iter().join(" "));
    println!("{}", bcnt.iter().join(" "));
}
