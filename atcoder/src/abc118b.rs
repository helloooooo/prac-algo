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
    let (n, m) = {
        let t = read_vec::<usize>();
        (t[0], t[1])
    };
    let v = read_vec2::<usize>(n as u32);
    let mut map = HashMap::new();
    for j in 0..n {
        for k in 1..v[j].len() {
            let y = map.get(&v[j][k]).map(|i| i + 1).unwrap_or(1);
            map.insert(v[j][k], y);
        }
    }
    let ans = map.into_iter().filter(|&(k, value)| value == n).count();

    println!("{}", ans);
}
