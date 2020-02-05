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
  let n: i8 = read();
  let tmp = read_vec::<String>();
  let (s, t) = {
    (&tmp[0], &tmp[1])
  };

  let s: Vec<char> = s.chars().collect();
  let t: Vec<char> = t.chars().collect();

  let mut ans: String = "".to_string();
  for i in 0..s.len() {
    ans.push_str(&(s[i].to_string()));
    ans.push_str(&(t[i].to_string()));
  }

  println!("{}", ans);
}