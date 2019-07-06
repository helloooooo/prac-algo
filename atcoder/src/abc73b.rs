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
    let n: u32 = read();
    let ln: Vec<Vec<i32>> = read_vec2(n);
    let mut ans = 0;
    for i in 0..n {
        ans += ln[i as usize][1] - ln[i as usize][0] + 1;
    }
    println!("{}", ans);
}