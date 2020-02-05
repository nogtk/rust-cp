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

  let s: Vec<char> = s.chars().collect();

  let mut count: i8 = 0;
  let mut s_last_index = s.len()-1;
  for i in 0..s.len()/2 {
    if s[i] != s[&s_last_index-i] {
      count += 1;
    }
  }

  println!("{}", count);
}