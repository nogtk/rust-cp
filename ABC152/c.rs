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
  let N: u64 = read();
  let p: Vec<u64> = read_vec();

  let mut min_elem: u64 = 200000 + 1;
  let mut ans = 0;
  for i in 0..p.len() {
    if(p[i] < min_elem) {
      min_elem = p[i];
      ans += 1;
    }
  }

  println!("{}", ans);
}
