use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    while n % 2 != 1 {
        n /= 2;
    }
    while n % 3 == 0 {
        n /= 3;
    }
    println!("{}", if n == 1 { "Yes" } else { "No" });
}
