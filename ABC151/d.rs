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

fn main() {
  let (h, w) = {
    let tmp: Vec<usize> = read_vec();
    (tmp[0], tmp[1])
  };
  let mut s: Vec<Vec<char>> = Vec::new();
  for _ in 0..h {
    let tmp: Vec<char> = read::<String>().chars().collect();
    s.push(tmp);
  }

  let mut deque: VecDeque<(usize, usize, usize)> = VecDeque::new();
  let mut max = 0;
  let mut tmp_max = 0;

  for i in 0..h {
    for j in 0..w {
      if s[i][j] == '#' { continue; }

      let mut dist: Vec<Vec<bool>> = vec![vec![false; w]; h];
      for i2 in 0..h {
        for j2 in 0..w {
          if s[i2][j2] == '#' {
            dist[i2][j2] = true;
          }
        }
      }

      deque.push_back((i, j, 0));
      dist[i][j] = true;

      loop {
        match deque.pop_front() {
          None => { break; }
          Some((x, y, d)) => {
            dist[x][y] = true;
            if x>0 && !dist[x-1][y] {
              dist[x-1][y] = true;
              deque.push_back((x-1, y, d+1));
              tmp_max = d+1;
            }
            if x<h-1 && !dist[x+1][y] {
              dist[x+1][y] = true;
              deque.push_back((x+1, y, d+1));
              tmp_max = d+1;
            }
            if y>0 && !dist[x][y-1] {
              dist[x][y-1] = true;
              deque.push_back((x, y-1, d+1));
              tmp_max = d+1;
            }
            if y<w-1 && !dist[x][y+1] {
              dist[x][y+1] = true;
              deque.push_back((x, y+1, d+1));
              tmp_max = d+1;
            }
          }
        }
      }
      if tmp_max > max { max = tmp_max; }
      tmp_max = 0;
    }
  }
  println!("{}", max);
}

