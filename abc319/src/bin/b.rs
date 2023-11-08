use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for i in 0..=n {
        let mut chk = true;
        for j in 1..10 {
            if n % j != 0 { continue; }
            if i % (n / j) == 0 {
                print!("{}", j);
                chk = false;
                break;
            }
        }
        if chk { print!("-"); }
    }
}
