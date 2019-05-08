fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
fn main() {
    let x = read::<f64>();
    let ans = revPow(x, 0);
    println!("{}", ans);
}
fn revPow(x: f64, c: i64) -> f64 {
    if c == 2 {
        return x;
    }
    revPow(x.sqrt(), c + 1)
}
