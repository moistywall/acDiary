use proconio::{input, marker::Usize1};

const INF: usize = 10000000000usize;

fn main() {
    input! {
        (n, m): (usize, usize),
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut dp = vec![vec![INF; n]; n];
    for (a, b, c) in abc.iter() {
        dp[*a][*b] = *c;
        dp[*b][*a] = *c;
    }
    for v in 0..n {
        dp[v][v] = 0;
    }

    // フロイド・ワーシャル法 ??
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }
}
