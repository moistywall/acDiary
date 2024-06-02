use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, cur: usize, seen: &mut Vec<bool>) {
    if seen[cur] {
        return;
    }
    seen[cur] = true;
    for next_v in g[cur].iter() {
        dfs(g, *next_v, seen);
    }
}

fn main() {
    input! {
        (n, m): (usize, usize),
    }
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut rg: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..m {
        input! { (a, b): (usize, usize) }
        g[a - 1].push(b - 1);
        rg[b - 1].push(a - 1);
    }
    let mut seen = vec![false; n];
    for i in 0..n {
        if !seen[i] { dfs(&g, i, &mut seen); }
    }
    let mut seen = vec![false; n];
    for i in 0..n {
        if !seen[i] { dfs(&rg, i, &mut seen); }
    }
}
