use proconio::{
    input,
};

fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    }
    let asum = a.iter().sum::<usize>();
    let bsum = b.iter().sum::<usize>();
    println!("{}", asum - bsum + 1);
}
