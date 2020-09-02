use proconio::{input, fastout};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input!(
        h: usize,
        w: usize,
        ch: usize,
        cw: usize,
        ds: usize,
        dw: usize,
        s: [String; h],
    );
    let s: Vec<Vec<char>> = s.iter().map(|ss| ss.chars().collect::<Vec<char>>()).collect();
    let dx = vec![1, 0, -1, 0];
    let dy = vec![0, 1, 0, -1];

    let mut dist = vec![vec![-1; w]; h];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    queue.push_back((ch-1, cw-1));

    dist[ch-1][cw-1] = 0;
    loop {
        match queue.pop_front() {
            Some((x, y)) => {
                let d = dist[x][y];
                for i in 0..4 {
                    let nx = x as isize + dx[i];
                    let ny = y as isize + dy[i];
                    if nx < 0 || ny < 0 { continue; }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx >= h || ny >= w || s[nx][ny] == '#' { continue; }
                    if dist[nx][ny] != -1 && dist[nx][ny] <= d { continue; }
                    queue.push_front((nx, ny));
                    dist[nx][ny] = d;
                }
                for ei in -2..3 {
                    for ej in -2..3 {
                        let nnx = x as isize + ei;
                        let nny = y as isize + ej;
                        if nnx < 0 || nny < 0 { continue; }
                        let nnx = nnx as usize;
                        let nny = nny as usize;
                        if nnx >= h || nny >= w || s[nnx][nny] == '#' { continue; } 
                        if dist[nnx][nny] != -1 && dist[nnx][nny] <= d+1 { continue; }
                        queue.push_back((nnx, nny));
                        dist[nnx][nny] = d+1;
                    }
                }
            },
            None => break,
        }
    }

    println!("{:?}", dist[ds-1][dw-1]);

}
