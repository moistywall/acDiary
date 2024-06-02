use proconio::{
    input,
    marker::Bytes,
};

fn main() {
    input! {
        n: usize,
        a: [Bytes; n],
        b: [Bytes; n],
    }
    for (i, (ai, bi)) in a.iter().zip(b.iter()).enumerate() {
        for (j, (aj, bj)) in ai.iter().zip(bi.iter()).enumerate() {
            if aj != bj {
                println!("{} {}", i + 1, j + 1);
                break;
            }
        }
    }
}
