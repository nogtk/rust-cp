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
  let d: Vec<u64> = read_vec();

  let mut sum = 0;
  for i in 0..d.len()-1 {
    for j in i+1..d.len() {
      sum += d[i] * d[j];
    }
  }

  println!("{}", sum);
}