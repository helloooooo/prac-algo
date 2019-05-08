use std::cmp::min;
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
    let n: i64 = read();
    let ans = (1..1000 as i64).fold(10000001, |y, x| {
        let sub = (1..1000 as i64)
            .filter(|a| x * a <= n)
            .fold(10000001, |a, b| min(a, (x - b).abs() + (n - x * b)));
        min(y, sub)
    });
    println!("{}", ans);
}
