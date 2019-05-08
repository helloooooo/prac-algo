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
    let sub_an: Vec<i64> = read_vec();
    let mut an = sub_an
        .iter()
        .fold(HashMap::new(), |mut map, x| {
            let y = map.get(&x).map(|i| i + 1).unwrap_or(1);
            map.insert(x, y);
            map
        })
        .into_iter()
        .flat_map(|(x, y)| {
            if y >= 4 {
                vec![x, x]
            } else if y >= 2 {
                vec![x]
            } else {
                vec![]
            }
        })
        .collect::<Vec<_>>();
    an.sort();
    an.reverse();
    let ans = if an.len() >= 2 { an[0] * an[1] } else { 0 };
    println!("{}", ans);
}
