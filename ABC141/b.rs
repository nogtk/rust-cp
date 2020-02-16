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


  let mut flag: bool = true;

  for i in 0..s.len() {
    if (i+1)%2 == 0 {
      if s[i] == 'R' { flag = false; break; }
    } else {
      if s[i] == 'L' { flag = false; break; }
    }
  }

  if flag { println!("Yes"); }
  else { println!("No"); }
}