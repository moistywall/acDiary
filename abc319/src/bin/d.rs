use proconio::{
    input,
};

// #[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        l: [usize; n],
    }
    let mut ok = 1 << 60;
    let mut ng = 0;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;

        let mut row = 0;
        let mut col = 0;
        for i in 0..n {
            if col + l[i] > mid {
                col = 0;
                row += 1;
            }
            col += l[i] + 1;
        }
        if row < m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

/* 解 https://atcoder.jp/contests/abc319/submissions/45366320 */