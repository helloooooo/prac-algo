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
    let nx = read_vec::<i64>();
    let (n, x) = (nx[0], nx[1]);
    let mut v = read_vec::<i64>();
    let mut ans = 0;
    for i in 1..n {
        let sum = v[i as usize] + v[(i - 1) as usize];
        if sum > x {
            let diff = x - sum;
            ans += diff.abs();
            v[i as usize] += diff;
        }
    }
    println!("{}", ans);
}
