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
    let nm = read_vec::<u64>();
    let (n, m) = (nm[0], nm[1]);
    let v = read_vec2::<u64>(m as u32);
    let ans = v.into_iter().fold(HashMap::new(), |mut map, x| {
        let y = map.get(&x[0]).map(|i| i + 1).unwrap_or(1);
        let z = map.get(&x[1]).map(|i| i + 1).unwrap_or(1);
        map.insert(x[0], y);
        map.insert(x[1], z);
        map
    });
    for m in 1..n + 1 {
        println!("{}", ans.get(&m).unwrap_or(&0));
    }
}
