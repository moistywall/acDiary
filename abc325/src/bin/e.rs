use proconio::{
    input,
};

fn main() {
    input! {
        (n, a, b, c): (usize, usize, usize, usize),
        d: [[usize; n]; n],
    }

    let inf = usize::MAX;
    let dijk = |sv: usize, bb: usize, cc: usize| -> Vec<usize> {
        let mut dist = vec![inf; n];
        dist[sv] = 0;
        let mut done = vec![false; n];
        for ni in 0..n {
            let mut best = (inf, 0);
            for (i, donei) in done.iter().enumerate() {
                if !donei { best = best.min((dist[i], i)); }
            }
            let v = best.1;
            done[v] = true;
            for i in 0..n {
                dist[i] = dist[i].min(dist[v] + d[v][i] * bb + cc);
            }
        }
        return dist;
    };

    let d1 = dijk(0, a, 0);
    let d2 = dijk(n - 1, b, c);
    let mut ans = inf;
    for i in 0..n {
        ans = ans.min(d1[i] + d2[i]);
    }
    println!("{}", ans);
}

// floyd_wrashall( https://atcoder.jp/contests/abc325/submissions/46844104 )