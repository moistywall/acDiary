use proconio::{
    input,
};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        st: [(usize, usize); n - 1],
    }

    for (i, (si, ti)) in st.iter().enumerate() {
        let num = a[i] / si;
        a[i + 1] += ti * num;
    }
    println!("{}", a[n - 1]);
}
