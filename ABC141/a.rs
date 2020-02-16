use std::collections::HashMap;
fn read<T: std::str::FromStr>() -> T {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
  read::<String>().split_whitespace()
    .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
  (0..n).map(|_| read_vec()).collect()
}

fn main() {
  let s: String = read();

  let mut hash = HashMap::new();
  let mut a: String = String::from("Sunny");
  let mut a_val: String = String::from("Sunny");
  let mut b: String = String::from("Cloudy");
  let mut b_val: String = String::from("Cloudy");
  let mut c: String = String::from("Rainy");
  let mut c_val: String = String::from("Rainy");

  hash.insert(a, b_val);
  hash.insert(b, c_val);
  hash.insert(c, a_val);

  println!("{}", hash.get(&s).unwrap());
}