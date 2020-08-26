use proconio::{input, fastout};
use std::cmp;

#[fastout]
fn main() {
    input!(
        n: usize,
        c: String,
    );

    let mut r2w = 0;
    let mut w2r = 0;

    for i in c.chars() {
        if i == 'R' { r2w += 1; }
    }

    let mut ans = cmp::max(r2w, w2r);
    let c: Vec<char> = c.chars().collect();

    for i in 0..n {
        if c[i] == 'R' { r2w -= 1; }
        else { w2r += 1; }
        ans = cmp::min(ans, cmp::max(r2w, w2r));
    }

    println!("{}", ans);
}