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

pub trait LexicalPermutation {
  /// Return `true` if the slice was permuted, `false` if it is already
  /// at the last ordered permutation.
  fn next_permutation(&mut self) -> bool;
}
impl<T> LexicalPermutation for [T] where T: Ord {
  /// Original author in Rust: Thomas Backman <serenity@exscape.org>
  fn next_permutation(&mut self) -> bool {
    // These cases only have 1 permutation each, so we can't do anything.
    if self.len() < 2 { return false; }

    // Step 1: Identify the longest, rightmost weakly decreasing part of the vector
    let mut i = self.len() - 1;
    while i > 0 && self[i-1] >= self[i] {
      i -= 1;
    }

    // If that is the entire vector, this is the last-ordered permutation.
    if i == 0 {
      return false;
    }

    // Step 2: Find the rightmost element larger than the pivot (i-1)
    let mut j = self.len() - 1;
    while j >= i && self[j] <= self[i-1]  {
      j -= 1;
    }

    // Step 3: Swap that element with the pivot
    self.swap(j, i-1);

    // Step 4: Reverse the (previously) weakly decreasing part
    self[i..].reverse();

    true
  }
}
fn main() {
  let n: usize = read();
  let x: Vec<isize> = read_vec();

  let x_min = x.iter().min().unwrap();
  let x_max = x.iter().max().unwrap();
  let mut min_sum = 1001001001001;

  for i in *x_min..*x_max {
    let mut sum = 0;
    for j in &x  {
      sum += ((i - j) as isize).pow(2);
    }
    if sum < min_sum { min_sum = sum; }
  }
  if min_sum == 1001001001001 { min_sum = 0; }
  println!("{}", min_sum);
}