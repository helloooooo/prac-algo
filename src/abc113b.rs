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
    let n = read::<f64>();
    let (t, a) = {
        let s = read_vec::<f64>();
        (s[0], s[1])
    };
    let hn = read_vec::<f64>();
    let sub: Vec<f64> = hn
        .into_iter()
        .map(|x| t - x * 0.006)
        .map(|x| (x - a).abs())
        .collect();
    let enu: Vec<_> = sub.into_iter().enumerate().collect();
    let mut ans = (0, 1e10 as f64);
    for (k, v) in enu {
        if v < ans.1 {
            ans = (k, v);
        }
    }
    println!("{}", ans.0 + 1);
}
