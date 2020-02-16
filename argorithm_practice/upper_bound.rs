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
  let array: Vec<usize> = read_vec();
  let key= 3;
  println!("{:?}", lower_bound(array, key));
}

fn lower_bound(arr: Vec<usize>, key: usize) -> usize {
  let mut left: usize = 0;
  let mut right: usize = arr.len();
  while left != right {
    let mid = (right + left) / 2;
    if arr[mid] <= key {
      left = mid+1;
    } else {
      right = mid;
    }
  }
  left
}