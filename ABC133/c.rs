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
	let tmp: Vec<usize> = read_vec();
	let (l, r) = (tmp[0], tmp[1]);
	let mut l_mod = l % 2019;
	let mut r_mod = r % 2019;
	if r_mod < l_mod {
		let tmp = r_mod;
		r_mod = l_mod;
		l_mod = tmp;
	}
	if r-l >= 2019 {
		println!("0");
		return;
	}
	let mut min: usize = 1001001001;
	for i in l_mod..r_mod {
		for j in i+1 .. r_mod+1 {
			min = std::cmp::min((i*j)%2019, min);
		}
	}
	println!("{}", min);
}
