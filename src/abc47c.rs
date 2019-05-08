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
pub const M: i32 = 1000000007;
fn main() {
    let ans = solve();
    println!("{}", ans);
}
fn solve() -> i32 {
    let s = read::<String>();
    s.chars()
        .fold(
            (0, ' '),
            |x, y| if y != x.1 { (x.0 + 1, y) } else { (x.0, y) },
        )
        .0
        - 1
}
