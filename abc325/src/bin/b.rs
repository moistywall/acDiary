use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        wx: [(usize, usize); n],
    }
    let mut mtg = vec![0; 24];
    for (w, x) in wx {
        let mut st = x;
        for i in 0..24 {
            if (9..18).contains(&st) {
                mtg[i] += w;
            }
            st = (st + 1) % 24;
        }
    }
    let ans = mtg.iter().max().unwrap();
    println!("{}", ans);
}
