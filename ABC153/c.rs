#![allow(warnings)]
fn read<T: std::str::FromStr>() -> T {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
  read::<String>().split_whitespace()
    .map(|e| e.parse().ok().unwrap()).collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {g]
  (0..n).map(|_| read_vec()).collect()
}

fn main() {
  let (N, K) = {
    let tmp: Vec<u64> = read_vec();
    (tmp[0], tmp[1])
  };

  let mut h: Vec<u64> = read_vec();

  h.sort_by(|a, b| a.cmp(b));

  for i in 0..K { h.pop(); }

  println!("{}", h.iter().sum::<u64>());
}