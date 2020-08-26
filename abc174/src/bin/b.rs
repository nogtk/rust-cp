use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        n: usize,
        d: usize,
    }

    let mut ans = 0;

    for _ in 0..n {
        input! {
            x: isize,
            y: isize,
        }

        if (x*x + y*y) <= (d*d) as isize {
            ans += 1;
        }
    }

    println!("{}", ans);
}