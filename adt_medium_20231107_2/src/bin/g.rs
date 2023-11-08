use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
    }
    let mut d = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            input! {
                x: usize,
            }
            d[i][j] = x;
        }
    }
    let mut dp = vec![0usize; 1 << n];
    for b in 0..(1 << n) {
        let mut l = 0;
        for i in 0..n {
            if (b >> i) & 1 != 1 {
                l = i;
                break;
            }
        }
        for i in 0..n {
            if (b >> i) & 1 != 1 {
                let nb = b | (1 << l) | (1 << i);
                dp[nb] = dp[nb].max(dp[b] + d[l][i]);
            } 
        }
    }
    println!("{}", dp.pop().unwrap());
}
