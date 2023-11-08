use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        if a.windows(2).all(|ai| ai[0] == ai[1]) {
            "Yes"
        } else {
            "No"
        }
    );
}
