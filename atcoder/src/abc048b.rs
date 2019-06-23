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
    let abx = read_vec::<i64>();
    let ans = if abx[0] == 0 {
        1 + abx[1] / abx[2]
    } else {
        (abx[1] / abx[2]) - ((abx[0] - 1) / abx[2])
    };
    println!("{}", ans);
}
