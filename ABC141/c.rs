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
  let (n, k, q) = {
    let tmp: Vec<usize> = read_vec();
    (tmp[0], tmp[1], tmp[2])
  };
  let mut a: Vec<usize> = Vec::new();
  for i in 0..q { a.push(read()) }

  if k > q {
    for _ in 0..n {
      println!("Yes");
    }
    return;
  } else {
    let mut answerer_vector: Vec<isize> = Vec::new();
    for i in 0..n { answerer_vector.push((q-k) as isize * -1)}
    for i in 0..q {
      let tmp = a[i] - 1;
      answerer_vector[tmp] += 1;
    }
    for i in 0..n {
      if answerer_vector[i] > 0 { println!("Yes"); }
      else { println!("No"); }
    }
  }
}