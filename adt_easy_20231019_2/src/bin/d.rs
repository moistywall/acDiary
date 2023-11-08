use proconio::input;

fn main() {
    input! {
        (n, m, mut t): (usize, usize, isize),
        a: [isize; n - 1],
    }
    let mut xy = vec![0; n];
    for _ in 0..m {
        input! {
            (x, y): (usize, isize),
        }
        xy[x - 1] = y;
    }

    for (i, &ai) in a.iter().enumerate() {
        if t <= ai {
            println!("No");
            return;
        }
        t -= ai;
        t += xy[i + 1];
    }
    println!("Yes");
}
