use proconio::{
    input,
};

fn main() {
    input! {
        (n, k, p): (usize, usize, usize),
    }
    
    let maxdp = (p as i32 + 1).pow(k as u32) as usize;
    let mut dp = vec![vec![usize::MAX; maxdp]; n + 1];
    dp[0][0] = 0;

    let f = |now: usize, a: &Vec<usize>| -> usize {
        let mut v = a.clone();
        let mut now = now;
        for i in 0..k {
            v[i] += now % (p + 1);
            v[i] = v[i].min(p);
            now /= p + 1;
        }

        let mut ret = 0;
        let mut q = 1;
        for i in v {
            ret += i * q;
            q *= p + 1;
        }
        ret
    };
    
    for i in 0..n {
        input! {
            c: usize,
            a: [usize; k],
        }
        for now in 0..maxdp {
            dp[i + 1][now] = dp[i + 1][now].min(dp[i][now]);
            let next = f(now, &a);
            dp[i + 1][next] = dp[i + 1][next].min(dp[i][now].saturating_add(c));
        }
    }
    println!("{}", if dp[n][maxdp - 1] == usize::MAX { -1 } else { dp[n][maxdp - 1] as isize })
}
