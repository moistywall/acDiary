#![allow(dead_code)]

use std::collections::VecDeque;
use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        (h, w): (usize, usize),
        s: [Chars; h],
    }
    let mut seen = vec![vec![false; w]; h];
    let dir: Vec<isize> = vec![1, -1];
    for (i, si) in s.iter().enumerate() {
        for (j, sii) in si.iter().enumerate() {
            if *sii == '#' {
                seen[i][j] = true;
                for &d in dir.iter() {
                    let dx = j as isize + d;
                    let dy = i as isize + d;
                    if dx >= 0 && dx < w as isize { seen[i][dx as usize] = true; }
                    if dy >= 0 && dy < h as isize { seen[dy as usize][j] = true; }
                }
            }
        }
    }
    let mut que = VecDeque::new();
    let mut cnts = Vec::new();
    cnts.push(1);
    for i in 0..h {
        for j in 0..w {
            if seen[i][j] { continue; }
            let mut cnt = 1usize;
            seen[i][j] = true;
            for &d in dir.iter() {
                let dx = j as isize + d;
                let dy = i as isize + d;
                if dx >= 0 && dx < w as isize {
                    if !seen[i][dx as usize] {
                        que.push_back((i, dx as usize));
                    }
                }
                if dy >= 0 && dy < h as isize {
                    if !seen[dy as usize][j] {
                        que.push_back((dy as usize, j));
                    }
                }
            }
            while let Some((y, x)) = que.pop_back() {
                println!("{:?}", que);
                seen[y][x] = true;
                cnt += 1;
                for &d in dir.iter() {
                    let dx = x as isize + d;
                    let dy = y as isize + d;
                    if dx >= 0 && dx < w as isize {
                        if !seen[i][dx as usize] {
                            que.push_back((i, dx as usize));
                        }
                    }
                    if dy >= 0 && dy < h as isize {
                        if !seen[dy as usize][j] {
                            que.push_back((dy as usize, j));
                        }
                    }
                }
            }
            cnts.push(cnt);
        }
    }
    println!("{}", cnts.iter().max().unwrap());
}
