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
    let s: String = read();
    let cs: Vec<char> = s.chars().collect();
    let len = cs.len();
    for i in 1..len {
        let sub = len / 2 - i;
        let (f, l) = cs.split_at(sub);
        let (s, l) = l.split_at(sub);
        if f == s {
            println!("{}", sub * 2);
            break;
        }
    }
}
