use std::cmp::min;
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
    let ans = solve(n,x);
    println!("{}",ans);
}

fn solve(n:i64,x:i64) -> i64 {
    if n == 0 {
        return if x >= 1 { 1 } else { 0 };
    }

    let mut sum = 1 ;
    for _ in 0..n {
        sum = 3 + sum * 2;
    }
    if x == 1 {
        0
    } else if x <= sum /2 {
        solve(n-1,x-1)
    } else if x == sum /2 + 1 {
        solve(n-1,x-1) + 1
    } else {
        let mut p = 1;
        for _ in 0..n-1 {
            p = 1 + p *2;
        }
        solve(n-1,x - sum/2 - 1) + 1 + p
    }

}
