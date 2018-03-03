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
    let ab :Vec<i32>= read_vec();
    let (a,b) = (ab[0],ab[1]);
    let ans = if (a * b) % 2 == 0 {
        "Even".to_string()
    } else {
        "Odd".to_string()
    };
    println!("{}",ans);
}