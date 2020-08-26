use proconio::{input, fastout};

#[fastout]
fn main() {
    input!(
        n: usize,
        a: [usize; n],
    );

    let mut current_max = 0;
    let mut sum = 0;
    for ai in a {
        if current_max < ai {
            current_max = ai;
        }
        sum += (current_max - ai);
    }
    println!(
        "{}", sum
    );
}
