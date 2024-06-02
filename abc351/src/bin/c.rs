use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    let mut b = Vec::new();
    for ai in a {
        b.push(ai);
        let mut size = b.len();
        if size <= 1 { continue; }
        while b[size - 2] == b[size - 1] {
            let bl = b.pop().unwrap();
            b.pop();
            b.push(bl + 1);
            size = b.len();
            if size <= 1 { break; }
        }
    }
    println!("{}", b.len());
}
