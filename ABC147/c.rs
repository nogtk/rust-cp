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
	let mut a: Vec<usize> = Vec::new();
	let mut graph: Vec<Vec<isize>> = vec![vec![-1; n]; n];
	for i in 0..n {
		let count: usize = read();
		for _ in 0..count {
			let tmp: Vec<usize> = read_vec();
			graph[i][tmp[0] - 1] = tmp[1] as isize;
		}
	}

	let mut ans = 0;
	for bit in 0..(1 << n) {
		let mut d: Vec<isize> = vec![0; n];
		for i in 0..n {
			if bit & (1 << i) != 0 {
				d[i] = 1;
			}
		}

		let mut ok: bool = true;
		for i in 0..n {
			if d[i] == 1 {
				for j in 0..n {
					if graph[i][j] == -1 { continue; }
					if graph[i][j] != d[j] { ok = false; }
				}
			}
		}

		if ok {
			let mut tmp_count = 0;
			for i in 0..n {
				if d[i] == 1 { tmp_count += 1; }
			}
			ans = std::cmp::max(ans, tmp_count);
		}
	}

	println!("{}", ans);
}
