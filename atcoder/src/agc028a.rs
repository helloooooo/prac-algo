use std::collections::BTreeMap;
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
    let (n, m) = {
        let t = read_vec::<usize>();
        (t[0], t[1])
    };
    let s: Vec<char> = read::<String>().chars().collect();
    let t: Vec<char> = read::<String>().chars().collect();
    let lcmv = lcm(n, m);
    let mut mm = BTreeMap::new();
    for (i, c) in s.into_iter().enumerate() {
        mm.insert(lcmv / n * i, c);
    }
    let mut check_flag = true;
    for (i, c) in t.into_iter().enumerate() {
        if let Some(x) = mm.get(&(lcmv / m * i)) {
            if c != *x {
                check_flag = false;
            }
        }
    }
    let ans = if check_flag { lcmv as i64 } else { -1 };
    println!("{}", ans);
}

fn gcd(x: usize, y: usize) -> usize {
    if x % y == 0 {
        y
    } else {
        gcd(y, x % y)
    }
}
fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}
