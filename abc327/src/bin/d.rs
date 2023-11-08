use proconio::input;

fn dfs(g: &Vec<Vec<usize>>, v: usize, x: &mut Vec<Option<u8>>, xi: u8) -> bool {
    if let Some(cxi) = x[v] {
        return cxi != xi;
    }
    let now = (xi + 1) % 2;
    x[v] = Some(now);
    for next_v in &g[v] {
        if !dfs(g, *next_v, x, now) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize; m],
        b: [usize; m],
    }
    let mut g = vec![vec![]; n];
    for (&a, &b) in a.iter().zip(b.iter()) {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut x: Vec<Option<u8>> = vec![None; n];
    for v in 0..n {
        if x[v].is_none() {
            if !dfs(&g, v, &mut x, 0) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
