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
  let n: u8 = read();
  let s: String = read();

  let mut chars = Vec::new();
  for ch in s.chars() {
    let ascii_code: u8 = ch as u8 + n;
    if ascii_code <= 90 {
      chars.push(ascii_code as char);
    } else {
      let ascii_code = ascii_code - 26;
      chars.push(ascii_code as char);
    }
  }

  let ans: String = chars.into_iter().collect();
  println!("{}", ans);

}