use itertools::Itertools;
use proconio::{input, marker::Bytes};

const M: usize = 4000000;

fn main() {
    input! {
        _: usize,
        s: Bytes,
    }
    let s = s.iter().map(|si| (si - b'0') as usize).collect_vec();
    let mut cnt_s = vec![0; 10];
    for si in s {
        cnt_s[si] += 1;
    }

    let mut ans = 0usize;
    for i in 0..M {
        let mut p = i * i;
        let mut cnt_p = vec![0; 10];
        while p > 0 {
            let j = p % 10;
            p /= 10;
            cnt_p[j] += 1;
        }

        if cnt_s[0] < cnt_p[0] {
            continue;
        }
        if (1..=9).all(|k| cnt_p[k] == cnt_s[k]) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
