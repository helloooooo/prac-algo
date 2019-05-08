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
    let n: i64 = read();
    let mut ans = 0;
    for j in 1..n + 1 {
        let mut v: Vec<i64> = Vec::new();
        for k in 1..j + 1 {
            if j % k == 0 && j % 2 == 1 {
                v.push(k);
            }
        }
        if v.len() == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
