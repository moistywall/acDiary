use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut dp = vec![f64::MIN; n + 1];
    dp[0] = 0.0;
    for pi in p {
        for i in (0..n).rev() {
            if dp[i] == f64::MIN {
                continue;
            }
            let new = dp[i] * 0.9 + pi as f64;
            dp[i + 1] = dp[i + 1].max(new);
        }
    }
    let mut ans = f64::MIN;
    let mut den = 1.0;
    for (i, d) in dp.iter().enumerate().skip(1) {
        ans = ans.max(d / den - 1200.0 / (i as f64).sqrt());
        den = den * 0.9 + 1.0;
    }
    println!("{}", ans);
}
