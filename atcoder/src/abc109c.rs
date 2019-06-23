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
    let (n, x) = {
        let t = read_vec::<i64>();
        (t[0], t[1])
    };
    let xn = read_vec::<i64>();
    let xn: Vec<i64> = xn.into_iter().map(|a| (x - a).abs()).collect();
    let mut ans = xn[0];
    for v in xn {
        ans = gcd(std::cmp::max(ans, v), std::cmp::min(ans, v));
    }
    println!("{}", ans);
}
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
