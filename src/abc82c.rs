use std::collections::HashMap;
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
    let n: i32 = read();
    let an: Vec<i32> = read_vec();
    let ans = run(an, 0);
    println!("{}", ans);
}
fn run(an: Vec<i32>, count: i32) -> i32 {
    let mut map = HashMap::new();
    for x in an {
        let ins_v = match map.get(&x) {
            Some(y) => y + 1,
            None => 1,
        };
        map.insert(x, ins_v);
    }
    map.iter()
        .map(|(&m, &n)| {
            if m == n {
                0
            } else if m < n {
                n - m
            } else {
                n
            }
        })
        .fold(0, |sum, l| sum + l)
}
