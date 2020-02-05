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
  let a: i8 = read();
  let b: i8 = read();

  let num: Vec<i8> = [1, 2, 3].to_vec();

  let mut ans: i8 = 0;
  for n in num {
    if (a != n && b != n) {
      ans = n;
      break;
    }
  }
  println!("{}", ans);
}