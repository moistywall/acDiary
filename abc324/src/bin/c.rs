use itertools::Itertools;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        (n, t): (usize, Chars),
        s: [Chars; n],
    }

    let mut ans: Vec<usize> = Vec::new();
    for (i, si) in s.iter().enumerate() {
        let num = si.len() as i64;
        let num1 = t.len() as i64;
        let sa = num - num1;
        if sa.abs() > 1 { continue; }

        let mut cnt = 1i64;
        let mut k = 0usize;

        // 1文字だけ違う,もしくは同じ
        if sa == 0 {
            for (&ti, &sii) in t.iter().zip(si.iter()) {
                if ti != sii { cnt -= 1; }
            }
        }

        let num1 = num1 as usize;
        // １文字多い
        if sa == 1 {
            for (j, &sii) in si.iter().enumerate() {
                if num1 <= j - k {
                    cnt -= 1;
                    break;
                }
                if sii != t[j - k] {
                    cnt -= 1; 
                    k = 1;
                }
            }
        }

        // １文字少ない
        if sa == -1 {
            for (j, &sii) in si.iter().enumerate() {
                if sii != t[j + k] {
                    cnt -= 1; 
                    if k == 0 {
                        k = 1;
                        if sii != t[j + k] { cnt-= 1; }
                    }
                }
            }
        }

        if cnt >= 0 {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
