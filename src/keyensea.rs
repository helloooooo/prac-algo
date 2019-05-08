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
    let n = read_vec::<i32>();
    let f = n.contains(&1);
    let s = n.contains(&7);
    let t = n.contains(&9);
    let k = n.contains(&4);
    let ans = if f && s && t && t && k { "YES" } else { "NO" };
    println!("{}", ans);
}
