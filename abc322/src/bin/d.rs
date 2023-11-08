use std::convert::TryInto;

use itertools::Itertools;
use proconio::{
    input,
    marker::Bytes,
};

const N: usize = 4;
const NS: isize = N as isize;

type Grid = [[bool; N]; N];

fn read_grid() -> Grid {
    input! {
        grid: [Bytes; N],
    }

    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|c| c == b'#')
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

fn enumerate_patterns(mut grid: Grid) -> Vec<Grid> {
    let mut patterns = Vec::new();
    for _ in 0..4 {
        // 平行移動
        for dx in -NS..=NS {
            for dy in -NS..=NS {
                let mut is_valid = true;
                let mut new_grid = [[false; N]; N];
                for x in 0..N {
                    for y in 0..N {
                        if !grid[x][y] {
                            continue;
                        }

                        let nx = x.wrapping_add_signed(dx);
                        let ny = y.wrapping_add_signed(dy);
                        if nx >= N || ny >= N {
                            is_valid = false;
                            break;
                        }
                        new_grid[nx][ny] = true;
                    }
                }

                if is_valid {
                    patterns.push(new_grid);
                }
            }
        }
        // 回転
        grid = (0..4)
            .map(|i| {
                (0..4)
                    .map(|j| grid[N - 1 - j][i])
                    .collect_vec()
                    .try_into()
                    .unwrap()
            })
            .collect_vec()
            .try_into()
            .unwrap();
    }
    patterns
}

fn main() {
    let g1 = read_grid();
    let g2 = read_grid();
    let g3 = read_grid();

    for p1 in enumerate_patterns(g1) {
        for p2 in enumerate_patterns(g2) {
            for p3 in enumerate_patterns(g3) {
                let mut is_valid = true;
                for x in 0..N {
                    for y in 0..N {
                        let mut cnt = 0;
                        if p1[x][y] {
                            cnt += 1;
                        }
                        if p2[x][y] {
                            cnt += 1;
                        }
                        if p3[x][y] {

                        }
                    }
                }
                if is_valid {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
