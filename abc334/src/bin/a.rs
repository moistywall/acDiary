use proconio::{
    input,
};

fn main() {
    input! {
        (b, g): (usize, usize),
    }

    println!("{}", if b > g { "Bat" } else { "Glove" });
}
 