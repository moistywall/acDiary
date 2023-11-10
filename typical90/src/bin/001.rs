use proconio::input;

fn main() {
    input! {
        (n, l): (usize, usize),
        k: usize,
        mut a: [usize; n],
    }
    a.push(l);
    let f = |x| -> bool {
        let mut num = 0usize;
        let mut pre = 0usize;
        for ai in &a {
            if ai - pre >= x {
                num += 1;
                pre = *ai;
            }
        }
        num > k
    };
    let mut ok = 0usize;
    let mut ng = l;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok);
}

// 231110 x
