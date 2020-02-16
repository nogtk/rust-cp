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
  let (n, k) = {
    let tmp: Vec<u64> = read_vec();
    (tmp[0], tmp[1])
  };
  let mut h: Vec<u64> = read_vec();

  let mut ans = 0;
  for i in h {
    if i >= k { ans+=1; }
  }

  println!("{}", ans);

}