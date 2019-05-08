use std::collections::HashMap;

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
    let n: String = read();
    let origin: Vec<char> = n.chars().collect();
    let mut cp = origin.clone();
    cp.reverse();
    let ans = if cp == origin {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
    println!("{}", ans);
}
