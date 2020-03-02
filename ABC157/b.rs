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
	let bingo: Vec<Vec<usize>> = read_vec2(3);
	let n = read();
	let mut b: Vec<usize> = Vec::new();
	for _ in 0..n {
		b.push(read());
	}

	let mut bingo_result = vec![vec![false; 3]; 3];

	for bi in b {
		for i in 0..3 {
			for j in 0..3 {
				if bingo[i][j] == bi {
					bingo_result[i][j] = true;
				}
			}
		}
	}

	let mut flag = false;
	for i in 0..3 {
		if bingo_result[i][0] && bingo_result[i][1] && bingo_result[i][2] {
			flag = true;
			break;
		}
	}
	for i in 0..3 {
		if bingo_result[0][i] && bingo_result[1][i] && bingo_result[2][i] {
			flag = true;
			break;
		}
	}
	if bingo_result[0][0] && bingo_result[1][1] && bingo_result[2][2] {
		flag = true;
	}
	if bingo_result[2][0] && bingo_result[1][1] && bingo_result[0][2] {
		flag = true;
	}

	if flag {
		println!("Yes");
	} else {
		println!("No");
	}
}
