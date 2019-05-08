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
    let mut s: Vec<char> = read::<String>().chars().rev().collect();
    let v: Vec<Vec<char>> = vec!["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|x| x.chars().rev().collect())
        .collect();
    let mut flag = true;
    while s.len() > 0 {
        let check = v.iter().find(|&x| s.starts_with(x));
        if let Some(x) = check {
            s = s[x.len()..].to_vec();
        } else {
            flag = false;
            break;
        }
    }
    println!("{}", if flag { "YES" } else { "NO" });
}
