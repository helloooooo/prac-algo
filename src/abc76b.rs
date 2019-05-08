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
    let i1: i32 = read();
    let i2: i32 = read();
    let mut ori = 1;
    for i in 0..i1 {
        match ori <= i2 {
            true => ori *= 2,
            false => ori += i2,
        }
    }
    println!("{}", ori);
}
