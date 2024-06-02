use proconio::input;

fn main() {
    input! {
        mut h: isize,
    }
    h *= -1;
    let mut cnt = 1usize;
    let mut b = 1isize;
    while h < 0 {
        cnt += 1;
        b *= 2;
        h += b;
    }
    println!("{}", cnt);
}
