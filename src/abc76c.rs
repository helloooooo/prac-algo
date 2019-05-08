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
    let i1: String = read();
    let i2: String = read();
    let s = i1.chars().collect::<Vec<char>>();
    let t = i2.chars().collect::<Vec<char>>();
    let div = s.len() - t.len() + 1;
    let ans = (0..div)
        .filter_map(|i| check(&s, &t, i))
        .min()
        .unwrap_or("UNRESTORABLE".to_string());
    println!("{}", ans);
}
fn check(s: &Vec<char>, t: &Vec<char>, start: usize) -> Option<String> {
    let mut cp_s = s.clone();
    for i in 0..t.len() {
        if s[i + start] != '?' && s[i + start] != t[i] {
            return None;
        }
    }
    for i in 0..t.len() {
        if cp_s[i + start] == '?' {
            cp_s[i + start] = t[i];
        }
    }
    for i in 0..cp_s.len() {
        if cp_s[i] == '?' {
            cp_s[i] = 'a';
        }
    }
    Some(cp_s.into_iter().collect::<String>())
}
