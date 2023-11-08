use proconio::{
    input,
};

fn main() {
    input! {
        (n, m): (usize, usize),
        a: [usize; m],
    }

    let mut pre = 1usize;
    for ai in a {
        for i in pre..=ai {
            println!("{}", ai - i);
        }
        pre = ai + 1;
    }
}
