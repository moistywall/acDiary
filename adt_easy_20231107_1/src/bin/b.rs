use proconio::input;

fn main() {
    input! {
        (r, c): (usize, usize),
        mut a1: [usize; 2],
        mut a2: [usize; 2],
    }
    a1.append(&mut a2);
    println!("{}", a1[2 * (r - 1) + c - 1]);
}
