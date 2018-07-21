use std::cmp::max;
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
    let nm:Vec<i64> = read_vec();
    let n = nm[0];
    let m = nm[1];

    let ans = if n == 1 && m == 1{
        1
    } else if n == 1 || m == 1{
        let x = max(n,m); 
        x -2
    } else {
        (n - 2) * (m - 2)
    };
    println!("{}",ans);
}