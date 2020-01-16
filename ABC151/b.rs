use std::ops::Neg;
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
    let nkm = read_vec();
    let a: Vec<usize> = read_vec();
    let n: usize = nkm[0];
    let k: usize = nkm[1];
    let m: usize = nkm[2];
    let a_sum = a.iter().fold(0, |sum, a| sum + a);
    let ans: isize =  if (a_sum + k)/ &n < m {
        -1
    } else if (a_sum)/&n >= m{
        0
    } else {
        (m * &n - a_sum ) as isize
    };

    println!("{:?}", ans);
}