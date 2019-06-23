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
    let w: Vec<char> = read::<String>().chars().collect();
    let mut cp = w.clone();
    cp.sort();
    cp.dedup();
    let ans = if cp
        .iter()
        .all(|x| w.iter().filter(|&y| y == x).count() % 2 == 0)
    {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
