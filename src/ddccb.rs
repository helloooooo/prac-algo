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
    let mut x = n - 2;
    let mut ans = 0;
    while x > 0 {
        ans += x;
        x -= 2;
    }
    ans *= 2;
    if n % 2 != 0 {
        ans -= (n - 2);
    }

    println!("{}", ans);
}
