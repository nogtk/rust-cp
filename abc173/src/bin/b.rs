use proconio::{input, fastout};
use std::collections::HashMap;
#[fastout]
fn main() {
    input!(
        n: usize,
        s: [String; n],
    );

    let mut map = HashMap::new();

    map.insert(String::from("AC"), 0);
    map.insert(String::from("WA"), 0);
    map.insert(String::from("TLE"), 0);
    map.insert(String::from("RE"), 0);
    
    for si in s {
        let count = map.entry(si).or_insert(0);
        *count += 1;
    }

    println!("AC x {}", map.get(&String::from("AC")).unwrap());
    println!("WA x {}", map.get(&String::from("WA")).unwrap());
    println!("TLE x {}", map.get(&String::from("TLE")).unwrap());
    println!("RE x {}", map.get(&String::from("RE")).unwrap());
}
