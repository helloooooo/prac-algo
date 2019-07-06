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
    let nl = read_vec::<i64>();
    let (n, l) = (nl[0], nl[1]);
    let mut sn: Vec<String> = Vec::new();
    for _ in 0..n {
        sn.push(read::<String>());
    }
    sn.sort();
    for s in &sn {
        print!("{}", s);
    }
}