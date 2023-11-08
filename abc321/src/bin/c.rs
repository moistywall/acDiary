use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
    }

    let mut v: Vec<i128> = Vec::new();
    for i in 0..(1<<10) as i128 {
        let mut x = 0;
        for j in (0..10).rev() {
            if (i >> j & 1) != 0 { x = x * 10 + j; }
        }
        if x == 0 { continue; }
        v.push(x);
    }
    v.sort();
    println!("{}", v[n - 1]);
}
