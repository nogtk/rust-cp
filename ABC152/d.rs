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
  let n: u32 = read();
  let mut hash = HashMap::new();

  for i in 1..n+1 {
    let pair = pair(i);
    let count = hash.entry(pair).or_insert(0);
    *count += 1;
  }

  let mut ans: u64 = 0;

  for i in 1..n+1 {
    let pair = pair(i);
    let pair_pair = (pair.1, pair.0);
    match hash.get(&pair_pair) {
      Some(count) => ans += *count,
      None => ans += 0
    }
  }

  println!("{}", ans);
}

fn pair(i: u32) -> (u32, u32) {
  let last = i%10;
  let start: u32 = i.to_string().chars().nth(0).unwrap().to_digit(10).unwrap();
  return (start, last);
}