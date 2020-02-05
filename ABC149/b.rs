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
    let abk: String = read();
    let abk_array: Vec<u64> = abk.split_whitespace().map(|e| e.parse().ok().unwrap()).collect::<Vec<_>>();
    let a: u64 = abk_array[0];
    let b: u64 = abk_array[1];
    let k: u64 = abk_array[2];

    let a_res: u64;
    let b_res: u64;


    if (a + b) < k {
        a_res = 0;
        b_res = 0;
    } else {
        if a <= k {
            a_res = 0;
        } else {
            a_res = a - k;
        }

        if a_res == 0 {
            b_res = b - (k - a);
        } else {
            b_res = b;
        }
    }

    let mut ans: String = a_res.to_string() + " ";
    ans = ans + &b_res.to_string();

    println!("{}", ans);
}