use proconio::{
    input,
};

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 { return x; }
    gcd(y, x % y)
}

fn main() {
    input! {
        mut a: [usize; 3],
    }
    let ga = gcd(a[0], gcd(a[1], a[2]) );
    let ans = a[0] / ga + a[1] / ga + a[2] / ga - 3;
    println!("{ans}");
}
