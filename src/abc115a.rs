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
    let n = read::<i64>();
    let ans = if n == 22 {
        "Christmas Eve Eve Eve".to_string()
    } else if n == 23 {
        "Christmas Eve Eve".to_string()
    } else if n == 24 {
        "Christmas Eve".to_string()
    } else {
        "Christmas".to_string()
    };
    println!("{}", ans);
}
