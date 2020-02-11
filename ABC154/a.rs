#![allow(warnings)]
use std::collections::HashMap;

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
//  let (s, t) = {
//    let mut tmp: Vec<String> = read_vec();
//    (&tmp[0], "hoge")
//  };
  let tmp: Vec<String> = read_vec();
  let (a, b) = {
    let tmp: Vec<i8> = read_vec();
    (tmp[0], tmp[1])
  };
  let u: String = read();

  let s = &tmp[0];
  let t = &tmp[1];

  let mut hash = HashMap::new();
  hash.insert(s, a);
  hash.insert(t, b);

  let num: i8 = hash.get(&u).unwrap() - 1;
  hash.insert(&u, num);

  println!("{:?} {:?}", hash.get(&s).unwrap(), hash.get(&t).unwrap());
}