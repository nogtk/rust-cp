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
  let n: usize = read();
  let mut s: Vec<String> = Vec::new();
  for _ in 0..n {
    s.push(read());
  }
  let mut hash = HashMap::new();
  for i in s {
    let count = hash.entry(i).or_insert(0);
    *count += 1;
  }

  let mut sorted_hash: Vec<_> = hash.iter().collect();
  sorted_hash.sort_by(|a, b| b.1.cmp(a.1));

  let max_value = sorted_hash[0].1;

  let mut keys: Vec<&str> = Vec::new();
  for (k, v) in sorted_hash {
    if max_value == v { keys.push(k); }
    if max_value != v { break; }
  }

  keys.sort();

  for i in keys {
    println!("{}", i);
  }
}