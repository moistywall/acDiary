use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, h, x): (usize, usize, usize),
        p: [usize; n],
    }

    let need = x - h;
    for (i, &p) in p.iter().enumerate() {
        if need <= p {
            println!("{}", i + 1);
            return;
        }
    }
}
