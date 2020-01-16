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
    let n: i64 = read();
    let root_n: i64 = (n as f64).sqrt() as i64 + 1;
    let mut ans = n-1;
    for i in 2..root_n {
        if n%i == 0 {
            ans = std::cmp::min(ans, (i-1)+(n/i-1));
        }
    }
    println!("{}", ans);
}