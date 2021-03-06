
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
    let nm = read_vec::<i64>();
    let ans  = go(nm[0],nm[1]);
    println!("{}",ans);
}
fn go(n:i64 , m:i64) -> i64{
    if m <= 1{
        return 0;
    }
    if n * 2 >= m {
        return m/2;
    }
    let ans  = n + (m - n * 2) / 4;
    ans
}