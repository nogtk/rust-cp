use proconio::{input, fastout};

#[fastout]
fn main() {
    input! { mut k: usize }

    let mut sevens: usize = 7;

    for i in 0..k {
        if sevens % k == 0 { 
            println!("{}", i+1);
            return;
        } else {
            let modulo = sevens % k;
            sevens = modulo * 10 + 7;
        }
    }

    println!("-1");
}