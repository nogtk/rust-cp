use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(
        n: f64,
    );

    if (n as isize) < 1000 {
        println!("{}", (n as isize - 1000).abs());
        return;
    }

    let ans = if n as isize % 1000 == 0{
        0
    } else {
        1000 * ((n / 1000.0).ceil() as isize) - (n as isize)
    };

    println!("{}", ans);
}
