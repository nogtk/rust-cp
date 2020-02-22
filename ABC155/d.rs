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
  let a: Vec<i64> = read_vec();

  let mut result: Vec<i64> = Vec::new();

  for i in 0..a.len()-1 {
    for j in i+1..a.len() {
      result.push(a[i]*a[j]);
    }
  }

  result.sort();

  println!("{}", result[(k-1) as usize]);
}