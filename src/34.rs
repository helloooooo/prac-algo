
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
    let x = read::<i64>();
    let (div,remind) = (x/11 , x%11);
    let rem =  match remind {
        0 => 0,
        n if n <= 6 => 1,
        _ => 2,
    };
    let ans = div * 2 + rem;
    println!("{}",ans);
}
