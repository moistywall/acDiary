use proconio::{
    input,
};

fn solve() {
    input! {
        (n, mut x, mut k): (usize, usize, usize),
    }
    let f = |v: usize, d: usize| -> usize {
        if v > n { return 0; }
        let mut l = v;
        let mut r = v;
        for _ in 0..d {
            l <<= 1;
            r = r << 1 | 1;
            if l > n { return 0; }
        }
        r = r.min(n);
        r - l + 1
    };

    let mut ans = f(x, k);
    while x > 1 && k >= 2 {
        ans += f(x^1, k - 2);
        k -= 1;
        x >>= 1;
    }
    if k == 1 && x != 1 { ans += 1; }
    println!("{}", ans);
}

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}
