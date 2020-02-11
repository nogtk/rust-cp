#![allow(warnings)]
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
  // 入力
  let (n, k) = {
    let tmp: Vec<usize> = read_vec();
    (tmp[0], tmp[1])
  };
  let mut p: Vec<f64> = read_vec();

  // p について累積和を計算

  let mut p_ev: Vec<f64> = Vec::new();
  let mut p_ruiseki: Vec<u64> = Vec::new();

  for i in &p {
    p_ev.push((i+1.0)/2.0);
  }

  let mut p_ev_ruiseki : Vec<f64> = Vec::new();

  p_ev_ruiseki.push(0.0);

  for i in 1..p_ev.len()+1 {
    let tmp = p_ev[i-1] + p_ev_ruiseki[i-1];
    p_ev_ruiseki.push(tmp);
  }

  let mut ans = 0.0;
  for i in 0..n-k+1 {
    let tmp = p_ev_ruiseki[i+k] - p_ev_ruiseki[i];
    if tmp > ans { ans = tmp; }
  }
  println!("{}", ans);
}
