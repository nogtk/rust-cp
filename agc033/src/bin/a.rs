use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        h: usize,
        w: usize,
        a: [String; h],
    );

    let a: Vec<Vec<char>> = a.iter().map(|aa| aa.chars().collect::<Vec<char>>()).collect();
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut dist = vec![vec![-1; w]; h];

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new(); 

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                dist[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }

    loop {
        match queue.pop_front() {
            Some((x, y)) => {
                for i in 0..4 {
                    let nx = x as isize + dx[i];
                    let ny = y as isize + dy[i];
                    if nx < 0 || ny < 0 { continue; }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx >= h || ny >= w { continue; }
                    if dist[nx][ny] == -1 {
                        dist[nx][ny] = dist[x][y] + 1;
                        queue.push_back((nx, ny));
                    }
                }
            },
            None => break,
        }
    }

    let ans = dist.iter().map( |aa| aa.iter().max().unwrap() ).max().unwrap();
    println!("{:?}", ans);
}
