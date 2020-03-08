use std::collections::VecDeque;
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
	let s: String = read();
	let q: usize = read();
	let queries: Vec<Vec<String>> = read_vec2(q as u32);

	let mut s: VecDeque<char> = s.chars().collect();
	let mut reverse_flag: bool = false;

	for i in 0..queries.len() {
		if queries[i][0] == String::from("1") {
			if s.len() > 1 {
				reverse_flag = !reverse_flag
			}
		}  else {
			let target = queries[i][2].chars().nth(0).unwrap();
			if queries[i][1] == String::from("1") {
				if reverse_flag { s.push_back(target) }
				else { s.push_front(target) }
			} else {
				if reverse_flag { s.push_front(target) }
				else { s.push_back(target)}
			}
		}
	}

	if reverse_flag {
		for i in 0..s.len() {
			print!("{}", s[s.len()-1 - i])
		}
		println!();
	} else {
		for si in s {
			print!("{}", si);
		}
		println!();
	}
}
