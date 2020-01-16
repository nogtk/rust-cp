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
//    let s: Vec<isize> = read_vec();
    let a: u32 = read();
    let b: u32 = read();
//    let ans = if s[0] >= 1 && s[0] <= 9 && s[1] >= 1 && s[1] <= 9 {
//        s[0] * s[1]
//    } else {
//        -1
//    };
//    println!("{}", ans);
    println!("{}", a);
    println!("{}", b);
}