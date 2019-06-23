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
    let xab = read_vec::<i64>();
    let (x, a, b) = (xab[0], xab[1], xab[2]);
    let ans = if (-a + b) <= 0 {
        "delicious"
    } else if -a + b <= x {
        "safe"
    } else {
        "dangerous"
    };
    println!("{}", ans);
}
