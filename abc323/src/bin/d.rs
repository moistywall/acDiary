use std::collections::BTreeMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        sc: [(usize, usize); n],
    }

    let mut mp: BTreeMap<_, _> = sc.iter().copied().collect();

    let mut ans = 0usize;
    while let Some((s, c)) = mp.pop_first() {
        ans += c & 1;
        let n = c >> 1;
        if n > 0 {
            if let Some(x) = mp.get_mut(&(s * 2)) {
                *x += n;
            } else {
                mp.insert(2 * s, n);
            }
        }
    }
    println!("{}", ans);
}
