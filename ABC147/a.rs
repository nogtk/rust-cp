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
  let (a1, a2, a3) = {
    let tmp: Vec<i8> = read_vec();
    (tmp[0], tmp[1], tmp[2])
  };

  let ans: &str = {
    if (a1+a2+a3) >= 22 {
      "bust"
    } else {
      "win"
    }
  };

  println!("{}", ans);
}