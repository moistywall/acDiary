use proconio::{
    input,
    marker::Bytes,
};

fn main() {
    input! {
        n: Bytes,
    }


    for ni in n.windows(2) {
        if ni[0] <= ni[1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
