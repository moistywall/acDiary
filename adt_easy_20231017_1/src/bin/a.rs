use proconio::input;

fn main() {
    input! {
        s: String,
    }
    println!(
        "{}",
        if s == "Hello,World!".to_string() {
            "AC"
        } else {
            "WA"
        }
    );
}
