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

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
  (0..n).map(|_| read_vec()).collect()
}

fn main() {
  let s: String = read();
  let s: &str = &s;

  let wd = ["SUN","MON","TUE","WED","THU","FRI","SAT"];

  let mut index: usize = 0;
  for (i, val) in wd.iter().enumerate() {
    if s == wd[i] {
      index = i;
      break;
    }
  }

  println!("{}", 7-index);
}