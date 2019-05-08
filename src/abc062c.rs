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
    let (h, w) = (hw[0], hw[1]);
    // let h = max(hw[0],hw[1]);
    // let w = min(hw[0],hw[1]);
    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }
    let a1 = solve(h, w);
    let a2 = solve(w, h);
    println!("{}", min(a1, a2));
}
fn solve(h: u64, w: u64) -> u64 {
    let mut ans = h * w;
    for j in 1..h {
        let g = suqare(j, w);
        let h_diff = h - j;
        let (b, r) = calc(h_diff, w);
        let mut v = vec![g, b, r];
        ans = min(v.iter().max().unwrap() - v.iter().min().unwrap(), ans);
        let (b, r) = calc(w, h_diff);
        let mut v = vec![g, b, r];
        ans = min(v.iter().max().unwrap() - v.iter().min().unwrap(), ans);
    }
    ans
}
fn suqare(x: u64, y: u64) -> u64 {
    x * y
}
fn calc(h: u64, w: u64) -> (u64, u64) {
    if h % 2 == 0 {
        let b = suqare(h / 2, w);
        (b, b)
    } else {
        let b = suqare(h / 2, w);
        let r = suqare(h - h / 2, w);
        (b, r)
    }
}
