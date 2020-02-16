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
  let mut L: Vec<usize> = read_vec();

  L.sort_by(|a, b| a.cmp(b));

  let mut ans = 0;

  for i in 0..n-2 {
    for j in i+1..n-1 {
      let ab = L[i] + L[j];
      let mut l = j;
      let mut r = n;
      while r-l > 1 {
        let m = (l+r)/2;
        if ab > L[m] {
          l = m;
        } else {
          r = m;
        }
      }
      ans += l-j;
    }
  }

  println!("{}", ans);
}