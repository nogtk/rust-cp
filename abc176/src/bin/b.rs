use proconio::{input, fastout};

#[fastout]
fn main() {
    input!( n: String );

    let s: Vec<char> = n.chars().collect();
    let sum = s.iter().map(|c| *c as i32 - 48 ).collect::<Vec<i32>>();
    let sum: i32 = sum.iter().sum();
    let ans = if sum % 9 ==0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
