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
    let s = read::<String>();
    let master = "keyence".to_string();
    for j in 0..s.len() {
        for k in j..s.len() {
            let a = &s[0..j];
            let b = &s[k..s.len()];
            if master == format!("{}{}", a, b) {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}
