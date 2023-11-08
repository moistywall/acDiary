use itertools::Itertools;
use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n :usize,
        mut td: [(usize, usize); n],
    }
    let mut v = td.iter().map(|(a, b)| (*a, a + b)).collect_vec();
    v.sort();
    let mut pq = BinaryHeap::new();
    let mut it = 0usize;
    let mut ans = 0usize;
    let mut i = 0usize;
    loop {
        if pq.is_empty() {
            if it == n {
                break;
            }
            i = v[it].0;
        }
        while it < n && v[it].0 == i {
            pq.push(Reverse(v[it].1));
            it += 1;
        }
        while !pq.is_empty() && pq.peek().unwrap() > &Reverse(i) {
            pq.pop();
        }
        if !pq.is_empty() {
            pq.pop();
            ans += 1;
        }
        i += 1;
    }
    println!("{}", ans);
}

// 他のよさそうな回答
// https://atcoder.jp/contests/abc325/submissions/46800091
// https://atcoder.jp/contests/abc325/submissions/46807679
