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
    let i: i32 = read();
    let c: Vec<i32> = i
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    let d: i32 = c.iter().sum();
    if i % d == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
