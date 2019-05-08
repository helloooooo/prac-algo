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
    let n: i64 = read();
    let an: Vec<i64> = read_vec::<i64>();
    let mut ans: Vec<(i64, i64)> = an
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut map, x| {
            map.insert((x.0 + 1) as i64, x.1);
            map
        })
        .into_iter()
        .collect();
    ans.sort_by_key(|k| k.1);
    ans.reverse();
    for j in &ans {
        println!("{}", j.0);
    }
}
