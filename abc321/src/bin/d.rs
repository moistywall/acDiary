use proconio::{
    input,
};

fn main() {
    input! {
        (n, m, p): (u128, u128, u128),
        a: [u128; n],
        mut b: [u128; m],
    }

    let mut ans = 0u128;
    let mut s = vec![0u128; (m + 1) as usize];
    b.sort();
    for (i, &b) in b.iter().enumerate() {
        s[i + 1] = s[i] + b;
    }

    for ai in a {
        let mut ok = m;
        let mut ng = 0;
        while ok != ng {
            let mid = (ok + ng) / 2;
            if ai + b[mid as usize] > p {
                ok = mid;
            } else {
                ng = mid + 1;
            }
        }
        ans += ai * ok;
        ans += p * (m - ok);
        ans += s[ok as usize];
    }
    println!("{}", ans);
}
