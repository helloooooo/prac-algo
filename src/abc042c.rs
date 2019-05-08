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
    let nk = read_vec::<i64>();
    let dn: Vec<char> = read_vec::<i64>()
        .iter()
        .map(|&x| x.to_string().chars().collect::<Vec<char>>()[0])
        .collect();
    for j in nk[0].. {
        if j.to_string().chars().all(|x| !dn.contains(&x)) {
            println!("{}", j);
            return;
        }
    }
}
