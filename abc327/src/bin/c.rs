use proconio::input;

fn main() {
    input! {
        mut row: [[usize; 9]; 9],
    }
    let mut col = vec![vec![]; 9];
    let mut masu = vec![vec![]; 9];
    for (i, ci) in row.iter().enumerate() {
        for (ii, &cii) in ci.iter().enumerate() {
            col[ii].push(cii);
            masu[(i / 3) * 3 + ii / 3].push(cii);
        }
    }
    for (r, (c, m)) in row.iter().zip(col.iter().zip(masu.iter())) {
        for i in 1..=9 {
            if !r.contains(&i) || !c.contains(&i) || !m.contains(&i) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
