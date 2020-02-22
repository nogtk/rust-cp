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
  let a: Vec<usize> = read_vec();

  let mut ans: &str = "APPROVED";

  for i in a {
    if i % 2 == 0 {
      if (i % 3 != 0) && (i % 5 != 0) {
        ans = "DENIED";
      }
    }
  }

  println!("{}", ans);
}