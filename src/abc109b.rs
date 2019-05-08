use std::collections::HashSet;
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
    let mut v = Vec::new();
    let mut set = HashSet::new();
    for _ in 0..n {
        v.push(read::<String>());
    }
    for x in &v {
        set.insert(x);
    }
    let len = set.len() as i64;
    if (n != len) {
        println!("No");
        return;
    }
    let sub = v.clone();
    let cs: Vec<Vec<char>> = sub.into_iter().map(|s| s.chars().collect()).collect();
    for j in 0..n - 1 {
        if cs[j as usize].iter().last().unwrap() != &cs[(j + 1) as usize][0] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
