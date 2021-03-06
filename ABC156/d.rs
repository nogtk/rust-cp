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
  let (n, a, b) = {
    let tmp: Vec<u64> = read_vec();
    (tmp[0], tmp[1], tmp[2])
  };

  let mut factorial_table: Vec<u64> = Vec::new();
  factorial_table.push(1);

  for i in 0..n {
    if i == 0 {
      factorial_table.push(1);
    } else {
      let tmp = factorial_table[i as usize];
      factorial_table.push((tmp * (i+1)));
    }
  }

  let mut sum: u64 = 0;
  for i in 1..n+1 {
    if i == a || i == b { continue; }
    sum += factorial_table[n as usize]/(factorial_table[i as usize]*factorial_table[(n-i) as usize]);
  }

  sum = sum % 1000000007;

  println!("{}", sum);
}