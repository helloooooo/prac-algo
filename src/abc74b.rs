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
    let n: i32 = read();
    let k: i32 = read();
    let xi: Vec<i32> = read_vec();
    let mut ans = 0;
    for i in 0..xi.len() {
        let n_div = (xi[i as usize] - 0).abs();
        let k_div = (xi[i as usize] - k).abs();
        if n_div >= k_div {
            ans += k_div * 2;
        } else {
            ans += n_div * 2;
        }
    }
    println!("{}", ans);
}
