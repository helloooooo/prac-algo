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
    let ab :Vec<String>= read_vec();
    let s = ab.concat();
    let sq = s.parse::<f64>().unwrap().sqrt();
    let ans = if sq.floor().powi(2) == s.parse().unwrap() {
        "Yes".to_string()
    } else{
        "No".to_string()
    };
    println!("{}",ans);
}