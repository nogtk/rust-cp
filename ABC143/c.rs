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
  let s: String = read();

  let mut ans = 0;
  let mut memory_char = ' ';
  for i in s.chars() {
    if i != memory_char {
      ans += 1;
      memory_char = i;
    }
  }
  println!("{}", ans);
}