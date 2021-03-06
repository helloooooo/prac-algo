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
    let abc = read_vec::<u64>();
    let (a, b, c) = (abc[0], abc[1], abc[2]);
    for j in 0..b + 1 {
        if (a * j) % b == c {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
