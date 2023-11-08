use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable_by(|a, b| b.cmp(a));
    let mut max = 0usize;
    let mut is = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut chk = a[i] + a[j];
            if chk % 2 == 0 {
                max = max.max(a[i] + a[j]);
                is += 1;
                break;
            }
        }
    }
    println!(
        "{}",
        if is == 0 {
            -1
        } else {
            max as isize
        }
    );
}
