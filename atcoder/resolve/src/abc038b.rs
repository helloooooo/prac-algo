
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
    let (h1, w1) = {
        let t = read_vec::<i64>();
        (t[0], t[1])
    };
    let (h2, w2) = {
        let t = read_vec::<i64>();
        (t[0], t[1])
    };
    let ans = if h1 == h2 || w1 == w2 || h1 == w2 || w1 == h2 {
        "YES".to_string()
    } else {
        "NO".to_string()
    };
    println!("{}", ans);
}
