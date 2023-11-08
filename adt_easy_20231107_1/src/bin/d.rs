use proconio::{
    input,
};

fn main() {
    input! {
        (a, b): (f64, f64),
    }
    let r = (a * a + b * b).sqrt();
    println!("{:.12} {:.12}", a / r, b / r);
}
