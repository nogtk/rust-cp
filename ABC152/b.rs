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
    let (a, b) = {
        let tmp = read_vec::<usize>();
        (tmp[0], tmp[1])
    };

    let min = std::cmp::min(&a, &b);
    let max: usize = std::cmp::max(a, b);
    let min_char = min.to_string();
    let mut ans: String = min_char.to_string();
    for i in 1..max {
        ans += &min_char;
    }

    println!("{}", ans);
}