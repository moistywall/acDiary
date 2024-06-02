use proconio::{
    input,
};

fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
        d: [usize; n],
    }

    let mut pl = vec![false; *d.last().unwrap()];
    for di in d {
        pl[di - 1] = true;
    }

    let mut cnt = 0usize;
    let mut ser = Vec::new();
    for pli in pl.windows(2) {
        cnt += 1;
        if pli[0] != pli[1] {
            ser.push(cnt);
            cnt = 0;
        }
    }
    ser.push(cnt + 1);
    println!("{:?}", ser);

}
