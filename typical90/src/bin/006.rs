use itertools::Itertools;
use proconio::{
    input,
    marker::Bytes,
};

fn main() {
    input! {
        (n, k): (usize, usize),
        s: Bytes,
    }
    let slen = s.len();
    let mut nex = vec![vec![0usize; 26]; 100009];
    for i in 0..26 {
        nex[slen][i] = slen;
    }
    for i in (0..slen).rev() {
        for j in 0..26 {
            if s[i] - b'a' == j as u8 {
                nex[i][j] = i;
            } else {
                nex[i][j] = nex[i + 1][j];
            }
        }
    }

    let mut ans = Vec::new();
    let mut cpos = 0usize;
    for i in 1..=k {
        for j in 0..26 {
            let npos = nex[cpos][j];
            let mposlen = slen as isize - npos as isize - 1 + i as isize;
            if mposlen >= k as isize {
                ans.push((b'a' + j as u8) as char);
                cpos = npos + 1;
                break;
            }
        }
    }
    println!("{}", ans.iter().join(""));
}
