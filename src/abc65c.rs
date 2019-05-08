const N: i64 = 1000000007;
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
    let nm: Vec<i64> = read_vec();
    let (n, m) = (nm[0], nm[1]);
    let ans = if (n - m).abs() < 2 && m == n {
        2 * (1..n + 1).fold(1, |x, y| (x * y) % N) * (1..m + 1).fold(1, |x, y| (x * y) % N)
    } else if (n - m).abs() < 2 {
        (1..n + 1).fold(1, |x, y| (x * y) % N) * (1..m + 1).fold(1, |x, y| (x * y) % N)
    } else {
        0
    };
    println!("{}", ans % N);
}
