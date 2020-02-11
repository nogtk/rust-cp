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
  let (a, b, x) = {
    let tmp: Vec<i64> = read_vec();
    (tmp[0], tmp[1], tmp[2])
  };

  let ans: i64 = binary_tree(a, b, x);

  println!("{}", ans);
}

fn binary_tree(a: i64, b: i64, x: i64) -> i64{
  let mut left = 0 as i64;
  let mut right = 1e9 as i64 + 1;
  while (right-left).abs() > 1 {
    let mid = (left + right) / 2;
    let value = mid;
    if is_true(a, b, x, value) {
      left = mid;
    } else {
      right = mid;
    }
  }
  left
}

fn is_true(a: i64, b: i64, x: i64, value: i64) -> bool {
  a * value + b * (value.to_string().len() as i64) <= x
}