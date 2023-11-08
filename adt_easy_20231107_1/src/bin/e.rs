use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort();
    let mut skip = false;
    let mut ans = 0usize;
    for ai in a.windows(2) {
        if skip {
            skip = false;
            continue;
        }
        if ai[0] == ai[1] {
            ans += 1;
            skip = true;
        }
    }
    println!("{}", ans);
}
