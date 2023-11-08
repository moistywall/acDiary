use proconio::{
    input,
    fastout,
};

fn dfs(g: &Vec<Vec<u128>>, n: usize, seen: &mut Vec<bool>) -> u128 {
    if seen.iter().all(|s| *s) { return 0u128; }
    let v = seen.iter().position(|&s| !s).unwrap();
    seen[v] = true;
    let mut ret = 0u128;
    for next_v in (v + 1)..n {
        if seen[next_v] { continue; }
        seen[next_v] = true;
        ret = ret.max(g[v][next_v] + dfs(g, n, seen));
        seen[next_v] = false;
    }
    seen[v] = false;
    ret
}

#[fastout]
fn main() {
    input! {
        mut n: usize,
    }
    let mut d: Vec<Vec<u128>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            input! {din: u128}
            d[i][j] = din;
            d[j][i] = din;
        }
    }

    let mut seen = vec![false; n];
    let mut ans = 0u128;
    if n % 2 == 0 {
        ans = dfs(&d, n, &mut seen);
    } else {
        for v in 0..n {
            seen[v] = true;
            ans = ans.max(dfs(&d, n, &mut seen));
            seen[v] = false;
        }
    }
    println!("{}", ans);
}
