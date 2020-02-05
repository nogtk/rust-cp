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
    let st: String = read();
    let arr = st.split_whitespace().map(|e| e.parse().ok().unwrap()).collect::<Vec<String>>();
    let s = arr[0].to_string();
    let t = arr[1].to_string();

    let ans: String = t+&s;

    println!("{}", ans);
}
