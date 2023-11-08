use proconio::{
    input,
    marker::Chars,
};

fn main() {
    input! {
        mut s: Chars,
    }

    while !s.is_empty() {
        let n = s.len() - 1;
        let mut chk = true;
        for i in 0..=(n / 2) {
            if s[i] != s[n - i] {
                chk = false;
                break;
            }
        }
        if chk {
            println!("{}", n + 1);
            return;
        }
        s.pop();
    }
}
