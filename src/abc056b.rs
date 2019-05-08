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
    let wab = read_vec::<i64>();
    let (w, a, b) = (wab[0], wab[1], wab[2]);
    let ans = if (a - b).abs() < w {
        0
    } else {
        (a - b).abs() - w
    };
    println!("{}", ans);
}
