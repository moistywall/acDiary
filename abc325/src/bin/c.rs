use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    let dx: Vec<isize> = vec![-1, 0, 1, 1, 1, 0, -1, -1];
    let dy: Vec<isize> = vec![1, 1, 1, 0, -1, -1, -1, 0];
    let mut seen = vec![vec![false; w]; h];
    let mut ans = 0usize;
    for j in 0..h {
        for i in 0..w {
            if s[j][i] == '.' || seen[j][i] {
                continue;
            }
            ans += 1;
            let mut deq = VecDeque::new();
            deq.push_back((i, j));
            seen[j][i] = true;
            while let Some((x, y)) = deq.pop_front() {
                let xx = x as isize;
                let yy = y as isize;
                for (&dx, &dy) in dx.iter().zip(dy.iter()) {
                    if xx + dx < 0 || yy + dy < 0 || xx + dx >= w as isize || yy + dy >= h as isize
                    {
                        continue;
                    }
                    if s[(yy + dy) as usize][(xx + dx) as usize] == '.'
                        || seen[(yy + dy) as usize][(xx + dx) as usize]
                    {
                        continue;
                    }
                    seen[(yy + dy) as usize][(xx + dx) as usize] = true;
                    deq.push_back(((xx + dx) as usize, (yy + dy) as usize));
                }
            }
        }
    }
    println!("{}", ans);
}
