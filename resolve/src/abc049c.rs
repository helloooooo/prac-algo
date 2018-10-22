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
fn main(){
    let v:Vec<Vec<char>> = ["dream","dreamer","erase","eraser"]
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect()
    let mut  s:Vec<char> = read::<String>().chars().rev().collect();
    let mut success = true;
    while s.len() > 0 {
        let matched = patterns.iter().find(|&p| s.starts_with(p));
        if let Some(p) = matched {
            s = &s[p.len()..];
        } else {
            succeeded = false;
            break;
        }
    }
    println!("{}", if succeeded { "YES" } else { "NO" });
}