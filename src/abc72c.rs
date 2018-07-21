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
    let ans = an.into_iter()
        .flat_map(|x| vec![x - 1, x, x + 1])
        .fold(HashMap::new(), |mut map, x| {
            let y = map.get(&x).map(|i| i + 1).unwrap_or(1);
            map.insert(x, y);
            map
        })
        .into_iter()
        .fold(None, |ori, (x, y)| match ori {
            None => Some((x, y)),
            Some((_, m)) if y > m => Some((x, y)),
            _ => ori,
        })
        .unwrap()
        .1;
    println!("{}", ans);
}

