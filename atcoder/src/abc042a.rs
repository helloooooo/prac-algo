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
    let s = read_vec::<i64>();
    let ans1 = s.iter().filter(|&x| *x == 7).count() == 1;
    let ans2 = s.iter().filter(|&x| *x == 5).count() == 2;
    println!("{}", if ans1 && ans2 { "YES" } else { "NO" });
}
