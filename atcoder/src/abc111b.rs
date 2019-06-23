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
    let s = read::<String>();
    let v = s.parse::<i64>().unwrap();
    let t: Vec<char> = s.clone().chars().collect();
    let first = t[0].to_digit(10).unwrap();
    let second = first + 1;
    let near_v = first * 100 + first * 10 + first;
    // let another_v = second*100 + second*10 + second;
    let ans = if near_v < v as u32 {
        second * 100 + second * 10 + second
    } else {
        near_v
    };
    println!("{}", ans);
}
