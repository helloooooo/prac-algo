use std::cmp::{max, min};
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
    let hw = read_vec::<u64>();
    let h = max(hw[0], hw[1]);
    let w = min(hw[0], hw[1]);
    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }
    let mut ans = h * w;
    for j in 1..h {
        let sg = suqare(j, w);
        let h_diff = h - j;
        let bg = suqare(h_diff, w / 2);
        let rg = suqare(h_diff, w - w / 2);
        let mut v = vec![sg, bg, rg];
        v.sort();
        ans = min(v[2] - v[0], ans);
    }
    println!("{}", ans);
}
fn suqare(x: u64, y: u64) -> u64 {
    x * y
}
