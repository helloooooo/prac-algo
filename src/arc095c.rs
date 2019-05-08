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
    let mut xn = read_vec::<i64>();
    let an = xn.clone();
    xn.sort();

    let medi = ((n - 2) / 2) as usize;
    let a = xn[medi];
    let b = xn[medi + 1];
    let ans: Vec<i64> = (0..n)
        .map(|k| if an[k as usize] <= a { b } else { a })
        .collect();
    for v in &ans {
        println!("{}", v);
    }
}
