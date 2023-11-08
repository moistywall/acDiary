use std::mem::swap;
use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        d: [usize; n],
        lck: [(usize, usize, usize); 2],
    }
    let inf = usize::MAX;
    let mut dp = vec![inf; lck[0].2 + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut p = vec![lck[0].2 + 1, inf];
        swap(&mut dp, &mut p);
        for x1 in 0..(lck[0].2 + 1) {
            let mut x2 = ((d[i] - lck[0].0 * x1) + lck[1].0 - 1) / lck[1].0;
            x2 = x2.max(0);
            for j in 0..(lck[0].2 + 1 - x1) {
                dp[j + x1] = dp[j + x1].min(p[j] + x2);
            }
        }
    }

    let mut ans = usize::MAX;
    for i in 0..(lck[0].2 + 1) {
        let j = dp[i];
        if j > lck[1].2 { continue; }
        let now = i * lck[0].1 + j * lck[1].2;
        ans = ans.min(now);
    }
    println!("{}", if ans == usize::MAX { -1 } else { ans as isize });
}
