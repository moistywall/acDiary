use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        (n, t): (usize, Chars),
        s: [Chars; n],
    }

    let tlen = t.len();
    let vl = s
        .iter()
        .map(|si| {
            let mut chk = 0usize;
            for &sii in si {
                if t[chk] == sii {
                    chk += 1;
                    if chk == tlen {
                        break;
                    }
                }
            }
            chk
        })
        .collect_vec();

    let mut vr = s
        .iter()
        .map(|si| {
            let mut chk = 0usize;
            for &sii in si.iter().rev() {
                if t[tlen - chk - 1] == sii {
                    chk += 1;
                    if chk == tlen {
                        break;
                    }
                }
            }
            chk
        })
        .collect_vec();

    vr.sort_by(|a, b| b.cmp(a));

    let mut ans = 0usize;
    for vli in vl {
        let mut ok = n;
        let mut ng = 0usize;
        while ok != ng {
            let mid = (ok + ng) / 2;
            if vr[mid] + vli >= tlen {
                ng = mid + 1;
            } else {
                ok = mid;
            }
        }
        ans += ok;
    }
    println!("{}", ans);
}
