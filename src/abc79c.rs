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
    let input: String = read();
    let v: Vec<i32> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    let ans = match (v[0], v[1], v[2], v[3]) {
        (a, b, c, d) if a + b + c + d == 7 => format!("{}+{}+{}+{}", a, b, c, d),
        (a, b, c, d) if a + b - c + d == 7 => format!("{}+{}-{}+{}", a, b, c, d),
        (a, b, c, d) if a + b + c - d == 7 => format!("{}+{}+{}-{}", a, b, c, d),
        (a, b, c, d) if a + b - c - d == 7 => format!("{}+{}-{}-{}", a, b, c, d),
        (a, b, c, d) if a - b + c + d == 7 => format!("{}-{}+{}+{}", a, b, c, d),
        (a, b, c, d) if a - b - c + d == 7 => format!("{}-{}-{}+{}", a, b, c, d),
        (a, b, c, d) if a - b + c - d == 7 => format!("{}-{}+{}-{}", a, b, c, d),
        (a, b, c, d) if a - b - c - d == 7 => format!("{}-{}+{}+{}", a, b, c, d),
        _ => unreachable!(),
    };
    println!("{}=7", ans);
}
