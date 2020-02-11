#![allow(warnings)]
use std::collections::HashSet;
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
  let n: u64 = read();

  let a: Vec<u64> = read_vec();

  let a_len = &a.len();

  let uniq: HashSet<u64> = a.into_iter().collect();

  let uniq_len = uniq.len();

  let ans = {
    if a_len == &uniq_len {
      "YES"
    } else {
      "NO"
    }
  };
  println!("{}", ans);
}
