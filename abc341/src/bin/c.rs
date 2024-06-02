use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        (h, w, _): (usize, usize, usize),
        t: Chars,
        s: [Chars; h],
    }

    let mut ans = 0;
    let dir = t.iter().map(|ti| {
        match ti {
            'L' => (0, -1),
            'R' => (0, 1),
            'U' => (-1, 0),
            'D' => (1, 0),
            _ => (0, 0),
        }
    }).collect::<Vec<(isize, isize)>>();
    for i in 1..(h - 1) {
        for j in 1..(w - 1) {
            if s[i][j] == '#' { continue; }
            let (mut hi, mut wi) = (i as isize, j as isize);
            let mut chk = true;
            for &(di, dj) in dir.iter() {
                hi += di;
                wi += dj;
                if s[hi as usize][wi as usize] == '#' {
                    chk = false;
                    break;
                }
            }
            if chk { ans += 1; }
        }
    }
    println!("{}", ans);
}
