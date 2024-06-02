use proconio::{
    input,
};

fn main() {
    input! {
        (n, q): (usize, usize),
        mut r: [usize; n],
    }
    r.sort();
    let mut ps = 0usize;
    let mut s = Vec::new();
    for &ri in r.iter() {
        ps += ri;
        s.push(ps);
    }
    for _ in 0..q {
        input! {
            x: usize,
        }
        let mut ok = n;
        let mut ng = 0;
        while ok != ng {
            let mid = (ok + ng) / 2;
            if s[mid] > x {
                ok = mid;
            } else {
                ng = mid + 1;
            }
        }
        println!("{}", ok);
    }
}
