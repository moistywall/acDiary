use proconio::{
    input,
    fastout,
};

#[fastout]
fn main() {
    input! {
        (n, d, p): (usize, usize, usize),
        mut f: [usize; n],
    }

    f.sort();
    let mut cnt = 0usize;
    let mut buff = 0usize;
    let mut temp = Vec::new();
    for f in f.iter().rev() {
        cnt += 1;
        buff += f;
        if cnt % d == 0 || cnt == n { 
            temp.push(buff); 
            buff = 0;
        }
    }
    let mut ans = 0usize;
    for t in temp {
        if t < p {
            ans += t;
        } else {
            ans += p;
        }
    }
    println!("{}", ans);
}
