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
    let n = read::<i64>();
    let ans = (1..)
        .take_while(|&x| x * x <= n)
        .filter(|&x| n % x == 0)
        .map(|x| max(get_x(x), get_x(n / x)))
        .min()
        .unwrap();

    print!("{}", ans);
}

fn get_x(x: i64) -> usize {
    let mut cnt = 0;
    let mut n = x;
    while n > 0 {
        cnt += 1;
        n /= 10;
    }
    cnt
}
