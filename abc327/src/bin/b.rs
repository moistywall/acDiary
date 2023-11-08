use proconio::input;

fn main() {
    input! {
        b: usize,
    }
    let mut chk = 1u32;
    loop {
        let p = (chk as usize).pow(chk);
        if p == b {
            println!("{}", chk);
            return;
        }
        if p > b {
            break;
        }
        chk += 1;
    }
    println!("-1");
}
