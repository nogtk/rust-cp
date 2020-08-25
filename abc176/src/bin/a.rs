use proconio::{input, fastout};

#[fastout]
fn main() {
    input! (
        n: usize,
        x: usize,
        t: usize,
    );

    let c = if (n%x == 0) {
        n/x
    } else {
        n/x + 1
    };

    println!("{}", t*c);
}
